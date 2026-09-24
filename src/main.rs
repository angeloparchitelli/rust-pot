use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;

const BIND_ADDRESS: &str = "0.0.0.0:2222";
const FAKE_BANNEER: &[u8] = b"SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5\r\n";

fn handle_connection(mut stream: TcpStream) {
    match stream.peer_addr() {
        Ok(peer_address) => {
            let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

            println!("[{}] [+] ALERT: Incoming connection from: {}", timestamp, peer_address);
        }
        Err(e) => eprintln!("[!] IP error {}",e),
    }

    if let Err(e) = stream.write_all(FAKE_BANNEER) {
        eprintln!("[!] Fake bunner error {}",e);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    println!("[++] Honeypot Running on {}", BIND_ADDRESS);

    
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("[!] Error {}",e);
            }
        }
    }
    Ok(())
}
