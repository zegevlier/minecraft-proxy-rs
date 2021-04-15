use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::io::Write;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use colored::*;
use env_logger::Builder;
use log::LevelFilter;

type DataQueue = deadqueue::unlimited::Queue<u8>;

// const CONNECT_IP: &str = "play.schoolrp.net:25565";
const CONNECT_IP: &str = "127.0.0.1:25565";
const BIND_ADDRESS: &str = "127.0.0.1:3333";

mod cipher;
mod functions;
mod types;

pub mod packet;
pub mod utils;

pub mod clientbound;
pub mod serverbound;

pub use packet::{Packet, Parsable};
pub use types::{Direction, State, Status};

async fn packet_parser(
    clientbound_queue: Arc<DataQueue>,
    direction: Direction,
    status: Arc<Mutex<Status>>,
) -> Result<(), ()> {
    let mut data = packet::Packet::new();
    let functions = functions::get_functions();
    loop {
        let new_byte = clientbound_queue.pop().await;
        let new_byte = match direction {
            Direction::Serverbound => status.lock().unwrap().server_cipher.decrypt(new_byte),
            Direction::Clientbound => status.lock().unwrap().client_cipher.decrypt(new_byte),
        };

        data.push(new_byte);
        while data.len() > 0 {
            let o_data: Vec<u8> = data.get();
            let packet_length = match data.decode_varint() {
                Ok(packet_length) => packet_length,
                Err(()) => {
                    data.set(o_data);
                    break;
                }
            };
            if (data.len() as i32) < packet_length {
                data.set(o_data);
                break;
            }
            let mut packet = packet::Packet::from(data.read(packet_length as usize).unwrap());
            if status.lock().unwrap().compress > 0 {
                let data_length = packet.decode_varint()?;
                if data_length > 0 {
                    let decompressed_packet = match decompress_to_vec_zlib(&packet.get()) {
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
            let packet_id = packet.decode_varint()?;
            // println!("{:?} {:X}", direction, packet_id);
            let mut parsed_packet = match functions
                .get(&direction)
                .unwrap()
                .get(&status.lock().unwrap().state)
                .unwrap()
                .get(&packet_id)
            {
                Some(func) => func.clone(),
                None => continue,
            };
            match parsed_packet.parse_packet(packet) {
                Ok(_) => {
                    let (packet_action, packet_info) = parsed_packet.get_printable();
                    log::info!(
                        "{} [{}]{3:4$} {}",
                        direction.to_string().yellow(),
                        packet_action.blue(),
                        packet_info,
                        "",
                        15 - packet_action.len()
                    )
                }
                Err(_) => {
                    log::error!("Could not parse packet!");
                    continue;
                }
            };
            if parsed_packet.state_updating() {
                parsed_packet
                    .update_status(&mut status.lock().unwrap())
                    .unwrap()
            }
        }
    }
}

async fn handle_connection(client_stream: TcpStream) -> std::io::Result<()> {
    let serverbound_queue = Arc::new(DataQueue::new());
    let clientbound_queue = Arc::new(DataQueue::new());
    let state: Arc<Mutex<Status>> = Arc::new(Mutex::new(Status::new()));
    log::info!("Connecting to {}...", CONNECT_IP);

    let server_stream = TcpStream::connect(CONNECT_IP).await?;
    let (mut srx, mut stx) = server_stream.into_split();
    let (mut crx, mut ctx) = client_stream.into_split();

    let sb_queue = serverbound_queue.clone();
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let n = match crx.read(&mut buf).await {
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
            for i in 0..n {
                sb_queue.push(buf[i]);
            }
            if let Err(e) = stx.write_all(&buf[0..n]).await {
                log::error!("failed to write to socket; err = {:?}", e);
                return;
            }
        }
    });

    let cb_queue = clientbound_queue.clone();
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let n = match srx.read(&mut buf).await {
                Ok(n) if n == 0 => {
                    log::warn!("Socket closed");
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    log::error!("Failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            for i in 0..n {
                cb_queue.push(buf[i]);
            }
            if let Err(e) = ctx.write_all(&buf[0..n]).await {
                log::error!("Failed to write to socket; err = {:?}", e);
                return;
            }
        }
    });

    let c_state = state.clone();
    tokio::spawn(async move {
        let client_queue = clientbound_queue.clone();
        packet_parser(client_queue, Direction::Clientbound, c_state)
            .await
            .unwrap();
    });

    let s_state = state.clone();
    tokio::spawn(async move {
        let server_queue = serverbound_queue.clone();
        packet_parser(server_queue, Direction::Serverbound, s_state)
            .await
            .unwrap();
    });

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Builder::from_default_env()
        // .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .format(|buf, record| {
            let formatted_level = buf.default_styled_level(record.level());
            writeln!(buf, "{:<5} {}", formatted_level, record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    log::info!("Starting listener...");
    let mc_client_listener = TcpListener::bind(BIND_ADDRESS).await?;

    loop {
        let (socket, _) = mc_client_listener.accept().await?;
        log::info!("Client connected...");
        handle_connection(socket).await?;
    }
}
