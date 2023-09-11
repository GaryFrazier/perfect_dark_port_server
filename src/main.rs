use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::io::Result;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use serde_derive::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use std::mem;
use lazy_static::lazy_static;
use std::sync::Mutex;

const BUFFER_SIZE: usize = 1024;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Mpsetup {
    
	/*0x800acb88*/ name: [char; 12],
	/*0x800acb94*/ options: u32,
	/*0x800acb98*/ scenario: u8,
	/*0x800acb99*/ stagenum: u8,
	/*0x800acb9a*/ timelimit: u8,
	/*0x800acb9b*/ scorelimit: u8,
	/*0x800acb9c*/ teamscorelimit: u16,

	/**
	 * Each bit signifies that a player or sim is participating.
	 *
	 * Bits 0x000f are for players
	 * Bits 0x0ff0 are for sims
	 * Bits 0xf000 are probably not used
	 */
	/*0x800acb9e*/ chrslots: u16,
	/*0x800acba0*/ weapons: [u8; 6],
	/*0x800acba6*/ paused: u8,
	/*0x800acba8*/ fileid: i32,
    deviceserial: u16
}


#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]

// A vector is 8 bytes with first byte being
struct Lobby {
    ip: Ipv4Addr,
    players: u8,
    max_players: u8,
    mpsetup: Mpsetup
}

lazy_static! {
    static ref GLOBAL_LOBBIES: Mutex<Vec<Lobby>> = Mutex::new(Vec::new());
}

#[allow(unreachable_code)]
fn main() -> Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        loop {
            // Your code that may result in an error goes here
            let result = get_packet(&socket);
            //println!("Packet retrieved");

            match result {
                Ok(_) => {
                    // The operation was successful, continue with the loop
                    //println!("Operation succeeded!");
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

        match ResponseType::from_u8(first_byte) {
            Some(ResponseType::GetLobby) => {
                //println!("Received response type {:?}", ResponseType::GetLobby);

                if let SocketAddr::V4(ipv4_addr) = src {
                    let ip_addr = ipv4_addr.ip();
                    let lobbies = GLOBAL_LOBBIES.lock().unwrap();
                    let bytes = serialize(&*lobbies).expect("Serialization failed");

                    //println!("{:?}", bytes);
                    socket.send_to(&bytes, &src)?;
                } else {
                    println!("Received data from a non-IPv4 address");
                }
            }
            Some(ResponseType::CreateLobby) => {
                let bytes_to_cast = &buf[1..(mem::size_of::<Mpsetup>())];
                let mpsetup: Mpsetup = deserialize(&bytes_to_cast).unwrap();

                if let SocketAddr::V4(ipv4_addr) = src {
                    let ip_addr = ipv4_addr.ip();
                    let mut lobbies = GLOBAL_LOBBIES.lock().unwrap();
                    let item_exists = lobbies.iter().any(|item| item.ip == *ip_addr);
                    // Push the new item if it doesn't exist
                    if !item_exists {
                        lobbies.push(Lobby { ip: *ip_addr, players: 3, max_players: 10, mpsetup: mpsetup });
                    } else {
                        println!("{:?} is already a lobby", *ip_addr);
                    }

                    //println!("{:?}", bytes);
                    socket.send_to(&buf, &src)?;
                } else {
                    println!("Received data from a non-IPv4 address");
                }

                println!("Received setup options {:?}", mpsetup);
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
