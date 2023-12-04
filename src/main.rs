use std::time::Duration;
#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};

fn main() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1))
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}

// fn f(a: &Cell<i32>, b: &Cell<i32>) {
//     let before = a.get();
//     b.set(b.get() + 1);
//     let after = a.get();
//     if before != after {
//         x();
//     } else {
//         println!("aa");
//     }
// }

fn x() {
    println!("x");
}
