use std::{net::TcpListener, io::{Write, Read, BufReader, prelude::*}, collections::HashMap};

fn get_method(db : & HashMap<String, i32>, key : String) -> i32{
    let maybe_ref = db.get(&key);
    let maybe = maybe_ref.cloned();
    maybe.unwrap_or(0)
}

fn set_method(db : & mut HashMap<String, i32>, key : String, x : i32) -> bool {
    let maybe_ref = db.insert(key, x);
    if maybe_ref.is_some() {
        return true;
    }
    false
}

fn write_response(str : String, a: i16) -> String {
    //let response = String::from("Hello, you are client #") + &a.to_string();
    //let response = format!("Hello, you are client #{a}");
    format!("{str} {a}\n")
}

fn main() {
    println!("Hello, world!");
    let mut a = 0;

    let mut database: HashMap<String, i32> = HashMap::new();
    
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    for stream in listener.incoming() {
        a += 1;
        let mut stream_no_err = stream.unwrap();
        
        loop {
            let mut reader = BufReader::new(&mut stream_no_err);
            let mut line = String::new();
            let Ok(read) = reader.read_line(&mut line) else { break; };

            if read == 0 {
                break;
            }

            let user_input = line.trim();
            dbg!(&user_input);

            let mut response = String::new();

            if user_input == "GET" {
                response = write_response("Get cmd".to_string(), a);
            }

            if user_input == "SET" {
                response = write_response("Set cmd".to_string(), a);
            }

            stream_no_err.write(response.as_bytes()).ok();
        }
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
    // let client = std::net::TcpStream::connect("127.0.0.1").unwrap();
}
