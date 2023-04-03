use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, fs,
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

    let response_status = "HTTP/1.1 200 OK";
    let file_content = fs::read_to_string("public/index.html").unwrap();
    let length = file_content.len();
    let content_lenght = format!("Content-Length: {length}");

    let response = format!("{response_status}\r\n{content_lenght}\r\n\r\n{file_content}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {:#?}", http_request)
}
