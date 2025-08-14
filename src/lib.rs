//! A `no_std`-compatible implementation of thread IDs and owned thread-local objects.
//!
//! Thread IDs are lazily initialized upon first access and can be accessed via:
//! ```
//! let current_thread_id = thid::ThreadId::current();
//! ```
//!
//! Thread-local objects can be created and passed around as follows:
//! ```
//! use std::{cell::RefCell, sync::Arc};
//!
//! let local_object = Arc::new(thid::ThreadLocal::<RefCell<String>>::new());
//!
//! let t1 = std::thread::spawn({
//!     let local_object = local_object.clone();
//!     move || {
//!         let local_value = local_object.get_or(|| RefCell::new("foo".into()));
//!         *local_value.borrow_mut() += "bar";
//!     }
//! });
//! let t2 = std::thread::spawn({
//!     let local_object = local_object.clone();
//!     move || {
//!         let local_value = local_object.get_or(|| RefCell::new("baz".into()));
//!         *local_value.borrow_mut() += "bip";
//!     }
//! });
//!
//! t1.join().unwrap();
//! t2.join().unwrap();
//!
//! let mut local_object = Arc::into_inner(local_object).expect("one remaining reference");
//! for mut_value in local_object.iter_mut() {
//!     let mut_value = mut_value.get_mut();
//!     *mut_value += "new";
//! }
//!
//! assert_eq!(
//!     local_object.iter_mut().map(|s| s.borrow().clone()).collect::<Vec<_>>(),
//!     vec!["foobarnew", "bazbipnew"],
//! );
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(feature = "std")), feature(thread_local))]

pub use thread_id::ThreadId;
pub use thread_local::ThreadLocal;

mod thread_id;
mod thread_local;
