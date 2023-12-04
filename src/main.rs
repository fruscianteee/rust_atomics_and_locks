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
    thread::spawn(|| dbg!(a));
    thread::spawn(|| dbg!(b));
}

fn f() {
    println!("hello from another thread!");
    let id = thread::current().id();
    println!("this is my thread id: {id:?}");
}
