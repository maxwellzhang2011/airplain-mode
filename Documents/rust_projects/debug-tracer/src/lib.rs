use std::sync::atomic::{AtomicUsize, Ordering};

static CALL: AtomicUsize = AtomicUsize::new(0);

pub fn tr(){
    let data = CALL.fetch_add(1, Ordering::Relaxed);
    println!("here {data}");
    return;
}
