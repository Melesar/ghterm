use std::thread::{self, ThreadId};
use std::sync::mpsc;

pub struct TaskManager {
    tasks: Vec<ThreadId>,
    sender: mpsc::Sender<ThreadId>,
    receiver: mpsc::Receiver<ThreadId>,
}

pub struct TaskHandle<T> {
    receiver: mpsc::Receiver<T>,
}

impl TaskManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<ThreadId>();
        TaskManager{ tasks: vec![], sender, receiver }
    }

    pub fn post<F, T>(&mut self, task: F) -> TaskHandle<T> 
        where F: Fn() -> T + Send + 'static,
              T: Send + 'static {

        let (sender, receiver) = mpsc::channel();
        let notify_sender = self.sender.clone();
        let handle = thread::spawn(move || {
            let result = (task)();
            sender.send(result).unwrap();
            notify_sender.send(thread::current().id()).unwrap();
        });
        self.tasks.push(handle.thread().id());
        TaskHandle{receiver}
    }

    pub fn update(&mut self) -> usize {
        if let Some(id) = self.receiver.try_recv().ok() {
            if let Some(index) = self.tasks.iter().position(|x| *x == id) {
                self.tasks.remove(index);
            }
        }

        self.tasks.len()
    }
}

impl<T> TaskHandle <T> {
    pub fn poll(&self) -> Option<T> {
        self.receiver.try_recv().ok()
    }
}
