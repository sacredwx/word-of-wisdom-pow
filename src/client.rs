#![warn(rust_2018_idioms)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use word_of_wisdom::{Challenge, CHALLENGE_SIZE};

use std::env;
use std::error::Error;
use std::net::SocketAddr;

const READ_BYTES_SIZE: usize = 256;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Reading the address from the arguments
    let addr: SocketAddr = env::args().nth(1).ok_or("Address is required")?.parse()?;

    // Connecting to the server
    let mut socket = TcpStream::connect(addr).await?;
    println!("Connected");

    // Receive the challlenge
    let mut buf = vec![0; CHALLENGE_SIZE];
    socket
        .read_exact(&mut buf)
        .await
        .expect("failed to read data from socket");

    // Create new challenge object
    let challenge =
        Challenge::new(bincode::deserialize(&buf).expect("failed to deserialize the challenge"));
    println!("Challenge received: {:?}", challenge.challenge);

    // Solve the challenge
    let (solution, tries) = challenge.solve();
    println!("Challenge solved: {:?} ({} iterations)", solution, tries);

    // Send the solution
    socket
        .write_all(&bincode::serialize(&solution).expect("failed to serialize the solution"))
        .await
        .expect("failed to write data to socket");

    // Read the very wanted row
    let mut received: Vec<u8> = vec![];
    loop {
        let mut buf: [u8; READ_BYTES_SIZE] = [0; READ_BYTES_SIZE];
        let bytes_read = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        received.extend_from_slice(&buf[..bytes_read]);

        if bytes_read < READ_BYTES_SIZE {
            break;
        }
    }
    let response: &str =
        bincode::deserialize(&received).expect("failed to deserialize the response");

    println!("{}", response);

    Ok(())
}
