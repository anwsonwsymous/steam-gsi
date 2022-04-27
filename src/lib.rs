use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::server::{GSIServer, SafeRequest};
use crate::utils::get_content_type;

pub mod dota;
pub mod server;
mod utils;

/// Supported game types
///
/// 1. Dota
/// 2. Counter Strike
pub enum Game {
    Dota,
    Cs,
}

/// Main struct which contains GSI Server and Listeners
pub struct GSI {
    server: Arc<Mutex<GSIServer>>,
}

impl GSI {
    pub fn new(game: Game, address: &str, trusted_token: &str) -> Self {
        Self {
            server: Arc::from(Mutex::new(GSIServer::new(game, address, trusted_token))),
        }
    }

    pub fn add_listener<F>(&mut self, listener: F)
        where F: Fn(SafeRequest) + Send + Sync + 'static
    {
        self.server.lock().unwrap().listeners.push(Arc::new(listener));
    }

    pub fn start_listening(&mut self) -> JoinHandle<()> {
        let gsi_server = self.server.clone();

        std::thread::spawn(move || {
            let gsi_server = gsi_server.lock().unwrap();

            for mut request in gsi_server.http.incoming_requests() {

                match gsi_server.parse_request(&mut request) {
                    Ok(json_request) => match gsi_server.handle_auth(&json_request) {
                        Ok(_) => {
                            for listener in &gsi_server.listeners {
                                let listener_clone = listener.clone();
                                let json_request_clone = json_request.clone();

                                std::thread::spawn(move || {
                                    listener_clone(json_request_clone);
                                });
                            }
                            gsi_server.respond_with_message(request, "Ok");
                        }

                        Err(message) => gsi_server.respond_with_message(request, message)
                    },
                    Err(message) => gsi_server.respond_with_message(request, message)
                }
            }
        })
    }
}