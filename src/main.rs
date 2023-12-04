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
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    // assert_eq!(a.as_ptr(), b.as_ptr());
    // thread::spawn(|| dbg!(a));
    // thread::spawn(|| dbg!(b));
    let a = RefCell::new(vec![10, 20]);
    dbg!(&a);
    f(&a);
    dbg!(&a);
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
