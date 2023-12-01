use std::time;

pub fn bench(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("Time taken {:?}", time::Instant::now().duration_since(t0));

    ret
}

