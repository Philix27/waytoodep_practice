use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::info;
pub struct DumbFuture {}

impl Future for DumbFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        info!("Hello from a dumb future!");
        // panic!("Oh heck no");
        Poll::Ready(())
    }
    // fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
    //     unsafe {
    //         *(0xF00D as *mut u64) = 0x0;
    //     }
    //     unreachable!(); // pinky promise
    // }
}
fn foo(x: Option<i32>) {
    match x {
        Some(n) if n >= 0 => println!("Some(Non-negative)"),
        Some(n) if n < 0 => println!("Some(Negative)"),
        Some(_) => unreachable!(), // compile error if commented out
        None => println!("None"),
    }
}
