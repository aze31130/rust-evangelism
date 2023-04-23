
use std::{net::{TcpListener, TcpStream}, io::{BufReader, prelude::*}, collections::HashMap, sync::{mpsc, mpsc::Sender}};

fn get_method(db : & HashMap<String, i32>, key : String) -> String{
    let maybe_ref = db.get(&key);
    let maybe = maybe_ref.cloned();
    let result = maybe.unwrap_or(0);
    write_response("Your value ", result)
}

fn set_method(db : & mut HashMap<String, i32>, key : String, x : i32) -> String {
    db.insert(key, x);
    write_response("Your value has been inserted", x)
}

fn write_response(str : impl ToString, a: i32) -> String {
    let str = str.to_string();
    format!("{str} {a}\n")
}

fn handle_cmd(db : & mut HashMap<String, i32>, user_input : &str) -> String {
    let mut response = String::new();

    let mut cmd = user_input.split(" ");

    let first = cmd.next();
    let second = cmd.next();
    let third = cmd.next();

    match [first, second, third] {
        [Some("GET"), Some(key), _] => {
            response = get_method(&db, key.to_string()).to_string();
        }
        [Some("SET"), Some(key), Some(value)] => {
            let value: i32 = value.parse().unwrap_or(-1);
            response = set_method(db, key.to_string(), value).to_string();
        }
        _ => {}
    }
    response
}

#[test]
fn main() {
    println!("Hello, world!");

    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let mut database = HashMap::new();
        loop {
            let (line, sender2): (String, Sender<String>) = receiver.recv().unwrap();
    
            let result: String = handle_cmd(& mut database, &line);
            sender2.send(result);
        }
    });
    
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    for stream in listener.incoming() {

        let mut stream = stream.unwrap();
        let sender = sender.clone();
        std::thread::spawn(move || {
            loop {
                let Some(line) = read_line(&mut stream) else { break; };
                let response = send_request(line, &sender);                
                stream.write(response.as_bytes()).ok();
            }
        });
    }
}

fn read_line(stream: &mut TcpStream) -> Option<String> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let Ok(read) = reader.read_line(&mut line) else { return None };
    if read == 0 {
        return None;
    }
    Some(line)
}

fn send_request(line: String, sender: &Sender<(String, Sender<String>)>) -> String {
    let (sender2, receiver2) = mpsc::channel();
    sender.send((line.trim().to_string(), sender2)).unwrap();
    receiver2.recv().unwrap()
}