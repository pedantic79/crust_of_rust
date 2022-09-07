use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// Flavors:
//   - Synchronous channels: Channel where send() can block. Limited capacity.
//     - Mutex + Condvar + VecDeque
//     - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//   - Asynchronous channels: Channel where send() cannot block. Unbounded.
//     - Mutex + Condvar + VecDeque
//     - Mutex + Condvar + LinkedList
//     - Atomic Queue (linked list), linked list of T
//     - Atomic block linked list, linked list of atomic VecDeque<T>
//   - Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization.
//   - Oneshot channels: Any capacity. In practice, only one call to send().

// https://doc.rust-lang.org/stable/std/sync/mpsc/fn.channel.html
// https://docs.rs/crossbeam-channel/latest/crossbeam_channel/
// https://docs.rs/flume/latest/flume/

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);

        if was_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);

        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();

        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    // if !inner.queue.is_empty() {
                    std::mem::swap(&mut self.buffer, &mut inner.queue);
                    // }
                    break Some(t);
                }
                None if inner.senders == 0 => break None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Mutex::new(Inner {
        queue: VecDeque::default(),
        senders: 1,
    });

    let shared = Arc::new(Shared {
        inner,
        available: Condvar::default(),
    });

    (
        Sender {
            shared: Arc::clone(&shared),
        },
        Receiver {
            shared: Arc::clone(&shared),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::channel;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();

        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
