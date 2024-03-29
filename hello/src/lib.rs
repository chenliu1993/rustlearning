use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


type Job = Box<dyn FnOnce() + Send +'static>;
impl ThreadPool {
    /// create pool
    /// the size of threads in the pool
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        } 
        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F> (&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job ; executing....", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
                // let job = 
                //     receiver.lock().unwrap().recv().unwrap();
                // println!("Worker {} got a job; executing.", id);
                // job();
            }  
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate msg to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
            
        println!("Shutting down all the workers");
        for worker in &mut self.workers {
            println!("Shutting down the worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                // join() has to take thread's ownship to function correctly.
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

