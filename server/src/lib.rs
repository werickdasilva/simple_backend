use std::{
    fs,
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
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else {
        ("HTTP/1.1 403 NOT FOUND", "public/404.html")
    };
    let file_content = fs::read_to_string(file).unwrap();
    let length = file_content.len();
    let content_lenght = format!("Content-Length: {length}");

    let response = format!("{status_line}\r\n{content_lenght}\r\n\r\n{file_content}");

    stream.write_all(response.as_bytes()).unwrap();
}
