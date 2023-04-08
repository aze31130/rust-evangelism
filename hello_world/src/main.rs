use std::{process::exit, io::Write};

use crate::maths::add;

mod voiture;
mod vehicule;
mod truck;
mod maths;
mod person;

// fn imprime_le_3e(data: &[char]) {
//     println!("{}",data[2]);
// }

#[test]
fn primitive() {
    let a = 35.;
    let b = 35;
    let str = "Hello";

    let ch = 'a';

    let mut table: &[i32] = &[1, 2, 3];

    table = &[4, 5, 6, 7];

    dbg!(table);

    // let s = String {
    //     len: 5,
    //     data: Box::new(['H', 'e', 'l', 'l', 'o']),
    // };

    if (false) {
        println!("True");
    } else {
        println!("False");
    }

    for i in 1..10 {
        println!("{i:?}");
    }

    for i in table {
        println!("{i:?}");
    }

    for i in str.chars() {
        println!("{i:?}");
    }

    let mut i = 0;
    while (i < 10) {
        if (i == 5) {
            break;
        }
        println!("{i}");
        i += 1;
    }

    loop {
        if (i == 10) {
            break;
        }
        println!("{i}");
        i += 1;
    }

}

#[test]
fn test_tableau() {

    fn récupère_le_7e(tableau: &[i32]) -> Option<i32> {

        let elem = tableau.get(6)?;
        Some(*elem)
    }

    dbg!(récupère_le_7e(&[1, 2, 3, 4]));
    dbg!(récupère_le_7e(&[1, 2, 3, 4, 5, 6, 7]));
}

// use maths::{add};
// use maths::*;
// use maths::add;

fn main() {

    let listener = std::net::TcpListener::bind(("localhost", 8080)).unwrap();

    loop {
        let (mut stream, address) = listener.accept().unwrap();
        println!("Connection from {address}");
        loop {
            let mut line = String::new();
            // stream.read_line(&mut line).unwrap();
            stream.write_all("hello world".as_bytes());
        }
    }
}
