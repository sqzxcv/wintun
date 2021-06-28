use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    let wintun = unsafe { wintun::load_from_path("examples/wintun/bin/amd64/wintun.dll") }
        .expect("Failed to load wintun dll");

    let adapter = match wintun::Adapter::open(&wintun, "Example", "Demo") {
        Ok(a) => a,
        Err(_) => {
            wintun::Adapter::create(&wintun, "Example", "Demo", None)
                .expect("Failed to create wintun adapter!")
                .adapter
        }
    };

    let session = Arc::new(adapter.start_session(wintun::MAX_RING_CAPACITY).unwrap());

    let reader_session = session.clone();
    let reader = std::thread::spawn(move || {
        while RUNNING.load(Ordering::Relaxed) {
            match reader_session.receive_blocking() {
                Ok(packet) => {
                    let bytes = packet.as_ref();
                    println!(
                        "Read packet size {} bytes. Header data: {:?}",
                        bytes.len(),
                        &bytes[0..20.min(bytes.len())]
                    );
                }
                Err(_) => println!("Got error while reading packet"),
            }
        }
    });
    println!("Press enter to stop session");

    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    println!("Shutting down session");

    RUNNING.store(false, Ordering::Relaxed);
    session.shutdown();
    let _ = reader.join();

    println!("Shutdown complete");
}