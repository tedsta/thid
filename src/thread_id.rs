use core::cell::Cell;
use core::sync::atomic::{AtomicU32, Ordering};

static NEXT_THREAD_ID: AtomicU32 = AtomicU32::new(0);

#[cfg(feature = "std")]
thread_local! {
    static THREAD_ID: Cell<ThreadId> = Cell::new(ThreadId::uninitialized());
}

#[cfg(not(feature = "std"))]
#[thread_local]
static THREAD_ID: Cell<ThreadId> = Cell::new(ThreadId::uninitialized());

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ThreadId(u32);

impl ThreadId {
    pub fn current() -> Self {
        let mut thread_id = THREAD_ID.get();

        if thread_id.is_uninitialized() {
            thread_id = ThreadId::next();
            THREAD_ID.set(thread_id);
        }

        thread_id
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    const fn uninitialized() -> Self {
        Self(u32::MAX)
    }

    fn next() -> Self {
        ThreadId(NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed))
    }

    fn is_uninitialized(&self) -> bool {
        self.0 == u32::MAX
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "std")]
    #[test]
    fn different_threads_have_different_ids() {
        use super::ThreadId;
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || tx.send(ThreadId::current()).unwrap())
            .join()
            .unwrap();

        let main_id = ThreadId::current();
        let other_id = rx.recv().unwrap();
        assert!(main_id != other_id);
    }
}
