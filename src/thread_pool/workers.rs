use super::jobs::Job;
use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct Worker<T> {
    id: usize,
    thread: JoinHandle<T>,
}

impl Worker<()> {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing...");
            job();
        });
        Worker { id, thread }
    }
}
