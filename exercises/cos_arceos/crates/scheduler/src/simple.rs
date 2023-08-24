use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

pub struct SimpleTask<T> {
    inner: T,
}

impl<T> SimpleTask<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
        }
    }

    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for SimpleTask<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct SimpleScheduler<T> {
    ready_queue: Arc<Mutex<LinkedList<SimpleTask<T>>>>,
}

impl<T> SimpleScheduler<T> {
    pub fn new() -> Self {
        SimpleScheduler {
            ready_queue: Arc::new(Mutex::new(LinkedList::new())),
        }
    }

    pub fn add_task(&mut self, task: SimpleTask<T>) {
        let mut queue = self.ready_queue.lock().unwrap();
        queue.push_back(task);
    }

    pub fn remove_task(&mut self, task: &SimpleTask<T>) -> Option<SimpleTask<T>> {
        let mut queue = self.ready_queue.lock().unwrap();
        let index = queue.iter().position(|t| t.inner() == task.inner()).unwrap();
        if let Some(index) = index {
            let removed_task = queue.remove(index);
            return Some(removed_task);
        }
        None
    }

    pub fn pick_next_task(&mut self) -> Option<SimpleTask<T>> {
        let mut queue = self.ready_queue.lock().unwrap();
        if let Some(task) = queue.pop_front() {
            return Some(task);
        }
        None
    }
}
