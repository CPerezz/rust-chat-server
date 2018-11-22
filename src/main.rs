extern crate dotenv;

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use dotenv::dotenv;
use std::env;
mod utils;

const MSG_SIZE: usize = 32;

fn main () {
    //Charge envoirment variables.
    dotenv().ok().expect(".env file not present");
    let host: String = env::var("HOST").expect("Error reading .env variable.");
    
    //Instanciate the server
    let server = TcpListener::bind(&host).expect("Listener failet to bind.");

    //Set server in Non-blocking state to force it to hear for changes all the time.
    server.set_nonblocking(true).expect("Failed to set Non-Blocking state.");

    //Generating a clients Storage Vector.
    let mut clients: Vec<std::net::TcpStream> = vec![];
    //An asynchronous, infinitely buffered channel. The channel function will return a (Sender, Receiver) tuple where all sends will be asynchronous.
    //Note that we've specified that Strings will be the types that travel through the channel.
    let (tx, rx) = mpsc::channel::<String>();
    loop{
        //If the recieved connection from the listener contains a correct value, is accepted.
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {:?} connected", addr);

            let tx = tx.clone();
            //Clone the socket to push it into a thread.
            clients.push(socket.try_clone().expect("Failed to clone the client."));

            thread::spawn(move || loop {
                //Create a buffer to store the msges.
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                    }
                }
            });

        }
    }


}
