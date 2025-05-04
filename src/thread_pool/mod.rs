use crate::Result;
pub use naive::NaiveThreadPool;
pub use rayon::RayonThreadPool;
pub use shared_queue::SharedQueueThreadPool;
use std::marker::Send;
mod naive;
mod rayon;
mod shared_queue;

pub trait ThreadPool {
    fn new(n: u8) -> Result<Self>
    where
        Self: Sized;

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
