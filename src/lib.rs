
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead};

pub struct Scheduler {
    listen_address: String,
    name: String,
}

impl Scheduler {
    pub fn new(listen_address: String, name: String) -> Scheduler {
        Scheduler {
            listen_address,
            name,
        }
    }

    fn listen(&self) {
        println!("Listening on {}", self.listen_address);
        let listener: TcpListener = TcpListener::bind(&self.listen_address).unwrap();
        for stream in listener.incoming() {
            println!("Connection established!");
            let mut stream: TcpStream = stream.unwrap();
            let mut reader = io::BufReader::new(&mut stream);
            let received: Vec<u8> = reader.fill_buf().expect("blah").to_vec();
            reader.consume(received.len());
            let data: String = String::from_utf8(received).unwrap();
            println!("Received: {:?}", data);
            
        }
    }

    pub fn run(&self) {
        println!("Scheduler is running");
        self.listen();
    }
}

struct Worker {
    id: usize,
    ip_address: String,
    name: String,
}

struct Message {
    id: usize,
    name: String,
    status: String,
}