use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::error::Error;
use std::sync::Arc;
use std::{env, fs};

use rand::seq::SliceRandom;

use word_of_wisdom::server_error::ServerError;
use word_of_wisdom::{Challenge, SOLUTION_SIZE};

const THE_BOOK: &str = "./word-of-wisdom.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the `word-of-wisdom` book
    let rows = Arc::new(load_the_book()?);

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (socket, _) = listener.accept().await?;

        // Clone the reference for the rows object in order to use asynchronously
        let rows = Arc::clone(&rows);

        tokio::spawn(async move {
            match handle_request(socket, rows).await {
                Err(e) => println!("{:?}", e),
                Ok(_) => println!("Connection successfully done"),
            }
        });
    }
}

async fn handle_request(mut socket: TcpStream, rows: Arc<Vec<String>>) -> Result<(), ServerError> {
    println!("Connection accepted");

    // Create new random challenge
    let challenge = Challenge::new_rand();
    println!("New challenge: {:?}", challenge.challenge);

    // Send the challenge
    socket
        .write_all(&bincode::serialize(&challenge.challenge).map_err(|_| ServerError::Serialize)?)
        .await
        .map_err(|_| ServerError::Communication)?;

    // Awaiting response
    let mut buf = vec![0; SOLUTION_SIZE];
    socket
        .read_exact(&mut buf)
        .await
        .map_err(|_| ServerError::Communication)?;
    // Receiving the solution
    let solution: [u8; SOLUTION_SIZE] =
        bincode::deserialize(&buf).map_err(|_| ServerError::Deserialize)?;
    println!("Solution received: {:?}", solution);

    // Check the solution for corectness
    let response = if challenge.check_solution(&solution) {
        println!("Solution {:?} correct!", solution);
        // Correct solution! Let's choose a row from the book!
        rows.choose(&mut rand::thread_rng())
            .ok_or(ServerError::DataSelectionError)?
    } else {
        // Bad solution!
        println!("Solution {:?} FAILED!!!", solution);
        "Trtying to cheat uh?!"
    };

    // Send the response
    socket
        .write_all(&bincode::serialize(response).map_err(|_| ServerError::Serialize)?)
        .await
        .map_err(|_| ServerError::Communication)?;

    Ok(())
}

fn load_the_book() -> Result<Vec<String>, Box<dyn Error>> {
    let mut rows: Vec<String> = vec![];
    for row in fs::read_to_string(THE_BOOK)?.split('\n') {
        rows.push(row.to_string());
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_new_from_file() {
        let rows = load_the_book().unwrap();

        let row = rows.choose(&mut rand::thread_rng()).unwrap();

        assert!(!row.is_empty());
    }
}
