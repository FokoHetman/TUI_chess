// FOK's GRILLIN'
// Feel free to edit how server works.
// However edited servers may not be accepted as official, without being monitored.


use std::{
  thread,
  sync::{mpsc, Arc, Mutex},
};


pub struct ThreadPool {
  worker_drones: Vec<WorkerDrone>,
  sender: mpsc::Sender<Job>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut worker_drones = Vec::with_capacity(size);

    for id in 0..size {
      worker_drones.push(WorkerDrone::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {worker_drones, sender}
  }
//  fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {

//  }
  pub fn execute<F>(&self, f: F)
   where F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}

struct WorkerDrone {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl WorkerDrone {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> WorkerDrone {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      //println!("\nWorker Drone {id} is handling new job...\n");
      job();
    });

    WorkerDrone {id, thread}
  }
}
