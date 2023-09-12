use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::io::Result;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use serde_derive::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use std::mem;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use std::convert::TryInto;
use std::ptr;

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

// A vector is 8 bytes with first byte being length of vec
struct Lobby {
    id: u8,
    players: u8,
    max_players: u8,
    mpsetup: Mpsetup
}

#[derive(Serialize, Deserialize)]

struct PlayerState {
    player_id: u32,
    x: f32,
    y: f32,
    z: f32
}

#[derive(Serialize, Deserialize)]

struct Gamestate {
    ready: bool,
    players: Vec<PlayerState>
}

lazy_static! {
    static ref GLOBAL_LOBBIES: Mutex<Vec<Lobby>> = Mutex::new(Vec::new());
    static ref GLOBAL_GAMESTATES: Mutex<HashMap<u8, Gamestate>> = Mutex::new(HashMap::new());  // lobby id/gamestate
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
    CreateLobby,
    JoinLobby
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
                let lobbies = GLOBAL_LOBBIES.lock().unwrap();
                let bytes = serialize(&*lobbies).expect("Serialization failed");

                socket.send_to(&bytes, &src)?;
            }
            Some(ResponseType::CreateLobby) => {
                let player_id = u32::from_be_bytes((&buf[2..6]).try_into().unwrap());
                let bytes_to_cast = &buf[6..(mem::size_of::<Mpsetup>())];
                let mpsetup: Mpsetup = deserialize(&bytes_to_cast).unwrap();

                if let SocketAddr::V4(ipv4_addr) = src {
                    let ip_addr = ipv4_addr.ip();
                    let mut lobbies = GLOBAL_LOBBIES.lock().unwrap();

                    let mut rng = rand::thread_rng();

                    // Generate a random u32
                    let lobby_id: u8 = rng.gen();

                    lobbies.push(Lobby { id: lobby_id, players: 1, max_players: 8, mpsetup: mpsetup });

                    let mut gameStates = GLOBAL_GAMESTATES.lock().unwrap();
                    gameStates.insert(lobby_id, Gamestate { ready: false, players: vec![PlayerState {player_id: player_id, x: 0.0, y: 0.0, z: 0.0}]});

                    // TODO ADD POLLING TO LOBBIES
                    buf[0] = lobby_id;
                
                    socket.send_to(&buf, &src)?;
                } else {
                    println!("Received data from a non-IPv4 address");
                }

                println!("Received setup options {:?}", mpsetup);
                socket.send_to(buf, &src)?;
            }
            Some(ResponseType::JoinLobby) => {
                let lobby_id = &buf[1];
                let player_id = u32::from_be_bytes((&buf[2..6]).try_into().unwrap());

                let mut gamestates = GLOBAL_GAMESTATES.lock().unwrap();
                
                // Search for a record with the specified value
                let mut found_gamestate: Option<&u8> = None;

                for (key, gamestate) in gamestates.iter_mut() {
                    if *key == *lobby_id {
                        gamestate.players.push(PlayerState { player_id: player_id, x: 0.0, y: 0.0, z: 0.0 });
                        found_gamestate = Some(key);
                        
                        break;
                    }
                }
                
                match found_gamestate {
                    Some(_) => {
                       
                    }
                    None => {
                        println!("lobby not found");
                    }
                }

                //println!("{:?}", bytes);
                socket.send_to(&buf, &src)?;
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
