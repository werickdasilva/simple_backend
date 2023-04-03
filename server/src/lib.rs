use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub fn app() -> App {
    App
}

pub struct App;

impl App {
    pub fn listen(&self, port: u32) {
        let localhost = format!("127.0.0.1:{}", port);
        let listen = TcpListener::bind(localhost).unwrap();

        for stream in listen.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream)        
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let reponse = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(reponse.as_bytes()).unwrap();
    println!("Request: {:#?}", http_request)
}
