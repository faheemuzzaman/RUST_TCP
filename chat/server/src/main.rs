extern crate rust_gpiozero;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use rust_gpiozero::*;

use std::thread::sleep;
use std::time::Duration; // note the capital D!


const LOCAL: &str = "192.168.1.37:6000";
const MSG_SIZE: usize = 2;

// fn sleep() {
//     thread::sleep(::std::time::Duration::from_millis(100));
// }

fn main() {
   // const led = LED::new(17);
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
       
        if let Ok((mut socket, addr)) = server.accept() {
            let led = LED::new(17);
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
            
			if msg == "1\r"{
			  led.on();
			}
			else if msg == "0\r"{
			  led.off();
			}
			else{
                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("failed to send msg to rx");
			}
                    }, 
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }

                // sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        // sleep();
    }

}
