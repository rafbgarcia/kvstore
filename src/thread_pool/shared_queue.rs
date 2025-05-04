use super::ThreadPool;
use crate::Result;
use std::thread;

pub struct SharedQueueThreadPool {}

impl ThreadPool for SharedQueueThreadPool {
    fn new(_n: u8) -> Result<SharedQueueThreadPool> {
        Ok(Self {})
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(job);
    }
}
