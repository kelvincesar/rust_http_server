use std::{thread, sync::mpsc};
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// type alias of trait object (allow us to make long types shorter)
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        
        // The Arc type will let multiple workers own the receiver, 
        // and Mutex will ensure that only one worker gets a job from
        // the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // receiver deve ter shared-ownership
            // smart pointer (usando Arc para ser thread safe)
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        // Copiado do thread::spawn
        // - FnOnce = a thread que irá executar uma requisição, irá executar apenas uma vez
        // - Send = transferir o clousure entre as threads
        // - 'static = Não sabemos quanto tempo uma thread irá demorar para executar;
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();

    }
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // Join para aguardar que todos processem a msg de finalização.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id, 
            thread: Some(thread) 
        }
    }
}