use std::sync::mpsc::sync_channel;
use std::sync::{Mutex, RwLock};

fn main() {
    let (sender, receiver) = sync_channel(1);
    sender.send(1).unwrap();

    let mut x = RwLock::new(10);

    let r = x.read().unwrap();
    let w = x.write().unwrap();

    let y = Mutex::new(20);

    let res = receiver.recv().unwrap();

    println!("Res is {}", res);
}