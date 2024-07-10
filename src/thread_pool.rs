use num_cpus;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() -> () + 'static + Send>;
enum Message {
    NewJob(Job),
    TerminateThread,
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let th = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv().unwrap();

            match msg {
                Message::NewJob(job) => {
                    println!("Worker #{} get new job!", id);
                    job();
                }
                Message::TerminateThread => {
                    println!("Worker #{} terminating...", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(th),
        }
    }
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let max_concurrent_proc = num_cpus::get();
        let mut workers: Vec<Worker> = Vec::with_capacity(max_concurrent_proc);

        println!("Spawn {} workers!", max_concurrent_proc);

        let (sender, receiver) = mpsc::channel::<Message>();
        let reciever = Arc::new(Mutex::new(receiver));

        for id in 1..max_concurrent_proc {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }

    pub fn exec<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::TerminateThread).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
