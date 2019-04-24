fn main() {
    let thread_n = 10_000;
    let mut threads = Vec::with_capacity(thread_n);

    for _ in 0..thread_n {
        let t = std::thread::Builder::new()
            .spawn(|| {
                loop {
                std::thread::sleep(std::time::Duration::from_secs(10));
                }
            })
            .unwrap();
        threads.push(t);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}