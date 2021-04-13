use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type DataQueue = deadqueue::unlimited::Queue<u8>;

// Client queue is messages clientbound
async fn client_parser(clientbound_queue: Arc<DataQueue>, state: Arc<Mutex<u8>>) {
    loop {
        let byte = clientbound_queue.pop().await;
        println!("S->C: {:x}", byte);
    }
    
}

// Server queue is mesages serverbound
async fn server_parser(serverbound_queue: Arc<DataQueue>, state: Arc<Mutex<u8>>) {
    loop {
        let byte = serverbound_queue.pop().await;
        println!("C->S: {:x}", byte);
    }
}

async fn handle_connection(client_stream: TcpStream) -> std::io::Result<()> {
    let serverbound_queue = Arc::new(DataQueue::new());
    let clientbound_queue = Arc::new(DataQueue::new());
    let state: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

    let server_stream = TcpStream::connect("127.0.0.1:4444").await?;
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
        client_parser(client_queue, c_state).await;
    });

    let s_state = state.clone();
    tokio::spawn(async move {
        let server_queue = serverbound_queue.clone();
        server_parser(server_queue, s_state).await;
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
