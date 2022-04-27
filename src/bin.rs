use dota_gsi::{Game, GSI};
use dota_gsi::server::SafeRequest;

fn listener_func(_: SafeRequest) {
    println!("Standard function as listener");
}

fn main() {
    let mut gsi = GSI::new(
        Game::Dota,
        "127.0.0.1:3903",
        "ahsd9f7a8sdfasd"
    );

    gsi.add_listener(|_| {
        println!("First listener");
    });
    gsi.add_listener(listener_func);

    let handle = gsi.start_listening();
    handle.join().unwrap();
}