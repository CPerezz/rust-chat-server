extern crate dotenv;

use std::io::{ErrorKind, Read, Write};
use std::time::Duration;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use dotenv::dotenv;
use std::env;
mod utils;

const MSG_SIZE: usize = 20;

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
    let (sender, reciever) = mpsc::channel::<String>();
    loop{
        //If the recieved connection from the listener contains a correct value, is accepted.
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {:?} connected to the chhanel!", addr);

            let sender = sender.clone();
            //Clone the socket to push it into a thread.
            clients.push(socket.try_clone().expect("Failed to clone the client."));

            thread::spawn(move || loop {
                //Create a buffer to store the msges.
                let mut buff = vec![0; MSG_SIZE];

                //Hear socket entries from sender an match it with a Result.
                match socket.read(&mut buff) {

                    //If read retunrs Ok Result
                    Ok(_) => {
                        //Set the buffer as an Iretartor and take it's elements while the condition retunrs true. Finally returns a Vec of type T
                        let msg = buff.clone().into_iter().take_while(|&x| x!= 0).collect::<Vec<_>>();

                        println!("\nMSG as Bytes:   {:?}", msg.clone());
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("\n{}: {:?}", addr, msg);
                        sender.send(msg).expect("failed to send msg to reciever");
                    }, 
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("\nClient: {} left the channel.", addr);
                        break;
                    }
                }

                thread::sleep(Duration::from_millis(200));
            });
        }

        if let Ok(msg) = reciever.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.clone().resize(buff.len(), 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        thread::sleep(Duration::from_millis(200));
    }
}
