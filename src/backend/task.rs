use std::thread;
use std::sync::mpsc;
use crate::app::events::AppEvent;

pub struct TaskManager {
    sender: mpsc::Sender<AppEvent>,
}

pub struct TaskHandle<T> {
    receiver: mpsc::Receiver<T>,
}

impl TaskManager {
    pub fn new(sender: mpsc::Sender<AppEvent>) -> Self {
        TaskManager{ sender }
    }

    pub fn post<F, T>(&mut self, mut task: F) -> TaskHandle<T> 
        where F: FnMut() -> T + Send + 'static,
              T: Send + 'static {

        let (sender, receiver) = mpsc::channel();
        let app_sender = self.sender.clone();
        thread::spawn(move || {
            let result = (task)();
            if let Ok(_) = sender.send(result) {
                app_sender.send(AppEvent::TaskCompleted).unwrap();
            }
        });
        TaskHandle{receiver}
    }
}

impl<T> TaskHandle <T> {
    pub fn poll(&self) -> Option<T> {
        self.receiver.try_recv().ok()
    }
}
