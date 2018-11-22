extern crate dotenv;

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::thread;
use dotenv::dotenv;
use std::env;
mod utils;


fn main () {
    dotenv().ok().expect(".env file not present");
    let host: String = env::var("HOST").expect("Error reading .env variable.");
    
    //let server = TcpListener::bind(&host)
}
