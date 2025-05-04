use super::ThreadPool;
use crate::Result;
use std::thread;

pub struct RayonThreadPool {}

impl ThreadPool for RayonThreadPool {
    fn new(_n: u8) -> Result<RayonThreadPool> {
        Ok(Self {})
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(job);
    }
}
