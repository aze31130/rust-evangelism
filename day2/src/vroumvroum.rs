use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;

#[test]
fn main_() {
    // let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let acc = 0;
    // v.par_iter_mut().for_each(|x| { acc += *x; *x += 1});

    let a = 0;
    let mutex = Arc::new(Mutex::new(a));

    let cloned_mutex = mutex.clone();
    let handle1 = thread::spawn(move | | {
        let mut value = cloned_mutex.lock().unwrap();
        *value += 1;

    });

    let cloned_mutex = mutex.clone();
    let handle2 = thread::spawn(move | | {
        let mut value = cloned_mutex.lock().unwrap();
        *value += 1;
        3
    });

    handle1.join().unwrap();
    let res = handle2.join().unwrap();
    let inner = Arc::try_unwrap(mutex).unwrap().into_inner().unwrap();
    dbg!(inner);
}

/*
pub fn other() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sorted = v.par_iter().sorted();
    println!("{:?}", sorted);
}
 */

enum DbCommand {
    Set(String, String),
    Get(String, mpsc::Sender<Option<String>>),
    Delete(String),
}

#[test]
pub fn test_sync() {
    let (db_sender, db_receiver) = mpsc::channel();

    let j2 = thread::spawn(move || {
        let mut db = HashMap::new();
        while let Ok(cmd) = db_receiver.recv() {
            match cmd {
                DbCommand::Set(key, value) => {
                    db.insert(key, value);
                }
                DbCommand::Get(key, tx) => {
                    let res = db.get(&key).cloned();
                    tx.send(res).unwrap();
                }
                DbCommand::Delete(key) => {
                    db.remove(&key);
                }
            }
        }
    });

    db_sender.send(DbCommand::Set("A".to_string(), "value of A".to_string())).unwrap();
    db_sender.send(DbCommand::Set("B".to_string(), "value of B".to_string())).unwrap();
    db_sender.send(DbCommand::Set("C".to_string(), "value of C".to_string())).unwrap();

    let (tx, rx) = mpsc::channel();
    db_sender.send(DbCommand::Get("B".to_string(), tx)).unwrap();
    let res = rx.recv().unwrap();
    dbg!(res);

    // struct Parent {}

    // struct Enfant {
    //     pub p: Parent
    // }

    

    j2.join().unwrap();
}

/*

Idée de projet:

Hashmap partagée par TCP

"set <key> <value>"


"get <key>"
<- "value"

 */