use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::thread;

fn ping_ip_address(ip: &str, port: u16) -> Result<(), std::io::Error> {
    let address = format!("{}:{}", ip, port);
    let timeout = Duration::from_secs(5); // Set a timeout of 5 seconds

    let addr = match address.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(err) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
        }
    };

    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => {
            //println!("Ping successful to {}:{}", ip, port);
            Ok(())
        }
        Err(err) => {
            //eprintln!("Error pinging {}:{}", ip, port);
            Err(err)
        }
    }
}

fn main() {

    let port = 80;
    
    let IPs = vec![
    "192.168.31.1",
    "192.168.31.15",
    "192.168.31.23",
    "192.168.31.68",
    "192.168.31.107",
    "192.168.31.123",
    "192.168.31.178",
    "192.168.31.202"];

    /*for ip in IPs {
        match ping_ip_address(ip, port) {
            Ok(_) => println!("{}:{} is ALIVE", ip, port),
            Err(err) => println!("{}:{} is DEAD", ip, port),
        }
    }*/

    // Create a vector to hold the children which are spawned.
    let mut children = vec![];

    for ip in IPs {
        // Spin up another thread
        children.push(thread::spawn(move || {
            match ping_ip_address(ip, port) {
                Ok(_) => println!("{}:{} is ALIVE", ip, port),
                Err(err) => println!("{}:{} is DEAD", ip, port),
            }
        }));
    }

    // Wait for the threads to finish. Return any errors encountered
    for child in children {
        let _ = child.join();
    }

    println!("Main thread is done");
}