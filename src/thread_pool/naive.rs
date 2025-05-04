use super::ThreadPool;
use crate::Result;
use std::thread;

pub struct NaiveThreadPool {}

impl ThreadPool for NaiveThreadPool {
    fn new(_n: u8) -> Result<NaiveThreadPool> {
        Ok(Self {})
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(job);
    }
}
