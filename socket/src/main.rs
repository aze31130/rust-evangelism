mod main2;

use std::{net::TcpListener, io::{Write, Read, BufReader, prelude::*}, collections::HashMap, sync::mpsc::Sender};

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
    //let response = String::from("Hello, you are client #") + &a.to_string();
    //let response = format!("Hello, you are client #{a}");
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
        // GET zieubzievb
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

fn main() {
    println!("Hello, world!");

    // let (sender, receiver) = std::sync::mpsc::channel();
    // let command = Command::Get { key: "zuvgyze".to_string(), result: sender };

    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut database: HashMap<String, i32> = HashMap::new();
        loop {
            let received: (String, Sender<String>) = receiver.recv().unwrap();
            let (line, sender2) = received;
    
            let result: String = handle_cmd(& mut database, line.trim());
            sender2.send(result);
        }
    });
    
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    for stream in listener.incoming() {

        let mut stream = stream.unwrap();
        let sender = sender.clone();
        std::thread::spawn(move || {
            loop {
                let mut reader = BufReader::new(&mut stream);
                let mut line = String::new();
                let Ok(read) = reader.read_line(&mut line) else { break; };
    
                if read == 0 {
                    break;
                }
                
                let (sender2, receiver2) = std::sync::mpsc::channel::<String>();
                sender.send((line.trim().to_string(), sender2)).unwrap();
                let response: String = receiver2.recv().unwrap();
    
                stream.write(response.as_bytes()).ok();
            }
        });
    }
}

/*
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
*/
/*

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

*/

#[test]
fn le_test_chill() {
    use std::sync::{Arc, Mutex};

    let pointeur_qu_on_peut_partager = Arc::new((3, "bonjoir"));
    let lock_qu_on_peut_partager = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for i in 0..10 {
        let copie = lock_qu_on_peut_partager.clone();
        let handle = std::thread::spawn(move || {
            let mut lock = copie.lock().unwrap();
            *lock += 1;
            println!("Hello, world! {i}");
            i
        });
        handles.push(handle);
    }

    for handle in handles {
        let le_i = handle.join();
    }

    dbg!(lock_qu_on_peut_partager.lock().unwrap());
    
    // let client = std::net::TcpStream::connect("127.0.0.1").unwrap();
}
