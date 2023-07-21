use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug, Default)]
pub struct Buffer {
    data: Mutex<Option<i32>>,
    is_empty: Condvar,
    is_full: Condvar,
}

pub fn newbuffer() -> Arc<Buffer> {
    Arc::new(Buffer::default())
}

impl Buffer {
    pub fn insert(&self, val: i32) {
        let mut lock = self.data.lock().expect("Can't lock");
        while lock.is_some() {
            lock = self.is_empty.wait(lock).expect("Can't wait");
        }
        *lock = Some(val);
        self.is_full.notify_one();
    }

    pub fn remove(&self) -> i32 {
        let mut lock = self.data.lock().expect("Can't lock");
        while lock.is_none() {
            lock = self.is_full.wait(lock).expect("Can't wait");
        }
        let val = lock.take().unwrap();
        self.is_empty.notify_one();
        val
    }
}
