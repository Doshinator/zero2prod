// - if we try to run two or more tests in parallel only one of them will manage to bind the port,
// all others will fail.

// - if port 8000 is being used by another program on our machine (e.g. our own application!), tests will fail;
// so this test will fail!!

use std::net::TcpListener;

pub fn spawn_app() -> String {
    let addrs: &str = "127.0.0.1:0";
    let listener = TcpListener::bind(addrs).expect("failed to bind address");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
