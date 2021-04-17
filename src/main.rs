use miniz_oxide::inflate::decompress_to_vec_zlib;
use parking_lot::Mutex;
use std::{io::Write, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
};

use colored::*;
use env_logger::Builder;
use log::LevelFilter;

type DataQueue = deadqueue::unlimited::Queue<Vec<u8>>;

mod cipher;
mod functions;
mod types;

pub mod packet;
pub mod utils;

pub mod clientbound;
pub mod serverbound;

pub use packet::{Packet, Parsable};
pub use types::{Direction, State, Status};

// This function starts a loop that parses all the recieved bytes into packets and then handels the packets
async fn packet_parser(
    queue: Arc<DataQueue>,
    direction: Direction,
    status: Arc<Mutex<Status>>,
    config: &types::ConfigFormat,
) -> Result<(), ()> {
    // It initializes a variable that will hold all the not yet parsed data
    let mut data = packet::Packet::new();
    // It then gets the functions that need to be called with each packet ID
    let functions = functions::get_functions();
    loop {
        // It gets a single byte from the queue
        let new_byte = queue.pop().await;
        // It then decrypts it with the correct cipher
        let new_byte = match direction {
            Direction::Serverbound => status.lock().server_cipher.decrypt(new_byte),
            Direction::Clientbound => status.lock().client_cipher.decrypt(new_byte),
        };

        // And then adds the byte to the list that still needs to be parsed
        data.push_vec(new_byte);
        // Then it does this loop until the queue is empty or until there is not enough data to parse the next packet.
        while data.len() > 0 {
            // It takes a backup of the data before trying to parse anything,
            // because there is a decent chance that the parsing fails and it needs to be restored.
            let o_data: Vec<u8> = data.get_vec();
            // It then starts parsing the packet by seeing the length the next packet will be.
            let packet_length = match data.decode_varint() {
                Ok(packet_length) => packet_length,
                Err(()) => {
                    data.set(o_data);
                    break;
                }
            };
            // If there is enough data to parse the packet, continue else break
            if (data.len() as i32) < packet_length {
                data.set(o_data);
                break;
            }
            // It then puts the data in a new object that should be empty at the end.
            let mut packet = packet::Packet::from(data.read(packet_length as usize).unwrap());
            // If the packet is compressed, decompress it and put it back in the object.
            if status.lock().compress > 0 {
                let data_length = packet.decode_varint()?;
                if data_length > 0 {
                    let decompressed_packet = match decompress_to_vec_zlib(&packet.get_vec()) {
                        Ok(decompressed_packet) => decompressed_packet,
                        Err(why) => {
                            log::error!("Decompress error: {:?}", why);
                            break;
                        }
                    };
                    packet.set(decompressed_packet);
                } else {
                    ()
                }
            }
            // Get the packet id
            let packet_id = packet.decode_varint()?;

            // Try to parse the packet with the packet ID, if the id is not found just continue to the next packet
            let mut parsed_packet = match functions
                .get(&direction)
                .unwrap()
                .get(&status.lock().state)
                .unwrap()
                .get(&packet_id)
            {
                Some(func) => func.clone(),
                None => continue,
            };

            // It then parses the packet with the found parser
            match parsed_packet.parse_packet(packet) {
                Ok(_) => {
                    // And prints the parsed packet data (with fancy colours)
                    let (packet_action, packet_info) = parsed_packet.get_printable();
                    if config.printing_packets.contains(&packet_action.to_string())
                        || config.printing_packets.contains(&"*".to_string())
                    {
                        log::info!(
                            "{} [{}]{3:4$} {}",
                            direction.to_string().yellow(),
                            packet_action.blue(),
                            packet_info,
                            "",
                            15 - packet_action.len()
                        )
                    }
                }
                Err(_) => {
                    // If it can't parse the packet just fail and move on
                    log::error!("Could not parse packet!");
                    continue;
                }
            };
            // It then updates the status if needed
            if parsed_packet.status_updating() {
                parsed_packet.update_status(&mut status.lock()).unwrap()
            }
        }
    }
}

async fn packet_listener(mut rx: OwnedReadHalf, mut tx: OwnedWriteHalf, queue: Arc<DataQueue>) {
    // This makes a buffer to hold all the sent bytes
    let mut buf = [0; 4096];
    loop {
        // It waits for bytes from the rx
        let n = match rx.read(&mut buf).await {
            Ok(n) if n == 0 => {
                log::warn!("Socket closed");
                return;
            }
            Ok(n) => n,
            Err(e) => {
                log::error!("failed to read from socket; err = {:?}", e);
                return;
            }
        };
        // Then adds them to the parsing queue (byte for byte)
        queue.push(buf[0..n].to_vec());
        // Then it sends them over to the tx
        if let Err(e) = tx.write_all(&buf[0..n]).await {
            log::error!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}

async fn handle_connection(
    client_stream: TcpStream,
    config: types::ConfigFormat,
) -> std::io::Result<()> {
    // It makes two queues that will hold all new packets.
    let serverbound_queue = Arc::new(DataQueue::new());
    let clientbound_queue = Arc::new(DataQueue::new());
    // It also makes a shared status that hold the current state + compression + ciphers
    let status: Arc<Mutex<Status>> = Arc::new(Mutex::new(Status::new()));
    log::info!("Connecting to {}...", &config.connect_ip);

    // This makes the connection to the actual server
    let server_stream = TcpStream::connect(&config.connect_ip).await?;
    // Then splits up both the connections in an rx and tx.
    let (srx, stx) = server_stream.into_split();
    let (crx, ctx) = client_stream.into_split();

    // It then starts a thread listening to new packets for both the tx and rx pairs.
    let sb_queue = serverbound_queue.clone();
    tokio::spawn(async move { packet_listener(crx, stx, sb_queue).await });

    let cb_queue = clientbound_queue.clone();
    tokio::spawn(async move { packet_listener(srx, ctx, cb_queue).await });

    // It also starts two threads to parse all the new packets both ways
    let c_status = status.clone();
    let c_config = config.clone();
    tokio::spawn(async move {
        packet_parser(
            clientbound_queue,
            Direction::Clientbound,
            c_status,
            &c_config,
        )
        .await
        .unwrap();
    });

    let s_status = status.clone();
    let s_config = config.clone();
    tokio::spawn(async move {
        packet_parser(
            serverbound_queue,
            Direction::Serverbound,
            s_status,
            &s_config,
        )
        .await
        .unwrap();
    });

    // Then it returns, because this is no longer needed
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load the logger, it has a fancy format with colours and it's spaced.
    Builder::from_default_env()
        .format(|buf, record| {
            let formatted_level = buf.default_styled_level(record.level());
            writeln!(buf, "{:<5} {}", formatted_level, record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    log::info!("Reading config...");
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let config = match settings.try_into::<types::ConfigFormat>() {
        Ok(config) => config,
        Err(err) => {
            panic!("Could not parse config file!\n{}", err)
        }
    };

    log::info!("Starting listener...");
    // Start listening on `BIND_ADDRESS` for new connections
    let mc_client_listener = TcpListener::bind(&config.listen_ip).await?;

    loop {
        // If this continues, a new client is connected.
        let (socket, _) = mc_client_listener.accept().await?;
        log::info!("Client connected...");
        // Start the client-handeling thread (this will complete quickly)
        handle_connection(socket, config.clone()).await?;
    }
}
