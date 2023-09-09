use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::io::Result;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use serde_derive::{Serialize, Deserialize};
use bincode::{serialize};

const BUFFER_SIZE: usize = 1024;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]

// A vector is 8 bytes with first byte being
struct Lobby {
    ip: Ipv4Addr,
    players: u8,
    max_players: u8,
    match_type_id: u8,
    map_id: u8,
    title: [char; 15]
}

#[allow(unreachable_code)]
fn main() -> Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        loop {
            // Your code that may result in an error goes here
            let result = get_packet(&socket);
            println!("Packet retrieved");

            match result {
                Ok(_) => {
                    // The operation was successful, continue with the loop
                    println!("Operation succeeded!");
                }
                Err(error) => {
                    // An error occurred, handle it as needed
                    println!("Error: {:?}", error);
                }
            }
        }
    } // the socket is closed here
    Ok(())
}

#[derive(Debug)]
#[derive(FromPrimitive)]
enum ResponseType {
    UNKNOWN,
    GetLobby,
    CreateLobby
}

fn get_packet(socket : &UdpSocket) -> Result<()> {
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; BUFFER_SIZE];
    let (res_bytes, src) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..res_bytes];

    if res_bytes > 0 {
        // Extract the first byte from the received data
        let first_byte = buf[0];
        let mut byte_index = 1; // we read first byte

        match ResponseType::from_u8(first_byte) {
            Some(ResponseType::GetLobby) => {
                println!("Received response type {:?}", ResponseType::GetLobby);

                if let SocketAddr::V4(ipv4_addr) = src {
                    let ip_addr = ipv4_addr.ip();
                    let lobbies =  vec![Lobby { ip: *ip_addr, players: 1, max_players: 8, match_type_id: 1, map_id: 1, title: ['T', 'E', 'S', 'T',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '] }, Lobby { ip: *ip_addr, players: 1, max_players: 8, match_type_id: 1, map_id: 1, title: ['T', 'E', 'S', 'T',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '] }];
                    let bytes = serialize(&lobbies).expect("Serialization failed");

                    println!("{:?}", bytes);
                    socket.send_to(&bytes, &src)?;
                } else {
                    println!("Received data from a non-IPv4 address");
                }
            }
            Some(ResponseType::CreateLobby) => {
                println!("Received response type {:?}", ResponseType::CreateLobby);
                socket.send_to(buf, &src)?;
            }
            _ => {
                println!("Received an unknown response type: 0x{:02X}", first_byte);
                // Handle the default case (any other byte)
            }
        }
    } else {
        println!("Received an empty response");
        // Handle the case where the response is empty
    }

    Ok(())
}
