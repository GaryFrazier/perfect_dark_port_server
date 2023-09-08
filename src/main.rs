use std::net::UdpSocket;
use std::io::Result;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

const BUFFER_SIZE: usize = 1024;

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
    GET_LOBBIES,
    CREATE_LOBBY
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
            Some(ResponseType::GET_LOBBIES) => {
                println!("Received response type {:?}", ResponseType::GET_LOBBIES);
                // Handle the 0x01 case
            }
            Some(ResponseType::CREATE_LOBBY) => {
                println!("Received response type {:?}", ResponseType::CREATE_LOBBY);
                // Handle the 0x02 case
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


    socket.send_to(buf, &src)?;

    Ok(())
}
