use miniz_oxide::inflate::decompress_to_vec;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type DataQueue = deadqueue::unlimited::Queue<u8>;

mod cipher;
mod functions;
pub mod packet;
mod types;

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
            Direction::Clientbound => status.lock().unwrap().client_cipher.decrypt(new_byte)
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
            if (data.len() as i32) <= packet_length {
                data.set(o_data);
                break;
            }
            let mut packet = packet::Packet::from(data.read(packet_length as usize).unwrap());
            if status.lock().unwrap().compress > 0 {
                let data_length = packet.decode_varint()?;
                if data_length > 0 {
                    let decompressed_packet = decompress_to_vec(&packet.get()).unwrap();
                    packet.set(decompressed_packet);
                } else {
                    ()
                }
            }
            let packet_id = packet.decode_varint()?;
            println!("{:?} {}", direction, packet_id);
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
                Ok(_) => println!("{}", parsed_packet.to_str()),
                Err(_) => {
                    println!("Could not parse packet!");
                    continue;
                }
            };
            if parsed_packet.state_updating() {
                parsed_packet.update_state(&mut status.lock().unwrap()).unwrap()
            }
        }
    }
}

async fn handle_connection(client_stream: TcpStream) -> std::io::Result<()> {
    let serverbound_queue = Arc::new(DataQueue::new());
    let clientbound_queue = Arc::new(DataQueue::new());
    let state: Arc<Mutex<Status>> = Arc::new(Mutex::new(Status::new()));

    let server_stream = TcpStream::connect("127.0.0.1:25565").await?;
    let (mut srx, mut stx) = server_stream.into_split();
    let (mut crx, mut ctx) = client_stream.into_split();

    let sb_queue = serverbound_queue.clone();
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let n = match crx.read(&mut buf).await {
                Ok(n) if n == 0 => {
                    println!("Socket closed");
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            for i in 0..n {
                sb_queue.push(buf[i]);
            }
            if let Err(e) = stx.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
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
                    println!("Socket closed");
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            for i in 0..n {
                cb_queue.push(buf[i]);
            }
            if let Err(e) = ctx.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
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
    let mc_client_listener = TcpListener::bind("127.0.0.1:3333").await?;

    loop {
        let (socket, _) = mc_client_listener.accept().await?;
        handle_connection(socket).await?;
    }
}
