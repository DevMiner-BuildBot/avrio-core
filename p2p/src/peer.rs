
use crate::tracker::*;
use crate::core::identity::*;
use rand::{Rand, Rng};
use crate::config::*;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::*;
use std::io::prelude::*;

pub struct Peer {
    id: Id,
    socket: Socket, // socket (ip, port) of a peer
    info: peer_tracker, // stats about recived and sent bytes from this peer
}

pub struct P2pdata {
    type: String, // The type of data in this message (eg Block would mean the deserialized data is a block struct)
    message_bytes: u64, // The lenght in bytes of this message
    message: String, // The serialized data
}

fn handle_client(mut stream: TcpStream) {
    let mut self_config : Config = config().unwrap();
    let buffer_bytes = self_config.buffer_bytes;
    let mut data = [0 as u8; buffer_bytes]; // using a config_self.buffer_bytes buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            //TODO: Code to verify handshake and then send our handshake needs to go HERE
            // Proccesss data recieved
            let mut data: P2pdata = serde_json::from_str(&data[0..size]);
            

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn rec_server() -> u8{
    let mut self_config = config();
    let listener = TcpListener::bind(self_config.bind_socket).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("peerlist.aio")
        .unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("[INFO] P2p incoming server launched on {0}", self_config.bind_socket);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                writeln!(file, stream.peer_addr().unwrap());
                println!("[INFO] New incoming connection: {0}", stream.peer_addr().unwrap());
                
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("[ERROR] handling peer connection to {0} resulted in  error {1}: {}", stream.peer_addr().unwrap(), e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
    return 1;
}
fn new_connection(socket: Socket) -> Peer { // This Fucntion handles all the details of conecting to a peer, geting id and constructing a Peer struct
    let mut peer = Peer;
    let mut peer_stream = TcpStream::connect(socket)?;
    let self_config = config().unwrap();
    /*Once we have established a connection over TCP we now send vital data as a hanshake,
    This is in the following format 
    0x1a, network id,our peer id, their peer id, our node type, ox1b;
    The recipitent then verifyes this then they send the same hand shake back to us;
    */
    let mut msg = "0x1a," + config.network_id + "," + serde_json::to_string(&self_config.identity).unwrap() + "," + Peer + "," + self_config.node_type + "," + config.network_id+ "0x1b";
    stream.write(serde_json::to_string(msg); // send our handshake
    stream.read(&mut [0; 128])?;
    Ok(())
}
