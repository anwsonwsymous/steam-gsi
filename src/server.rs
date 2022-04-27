use std::sync::Arc;
use tiny_http::{Request, Response, Server};
use crate::{dota, Game, get_content_type};

pub type SafeRequest = Arc<dyn GSIRequest + Send + Sync>;
pub type Listener = Arc<dyn Fn(SafeRequest) + Send + Sync>;

pub trait GSIRequest: std::fmt::Debug {
    fn authenticate(&self, trusted_token: &str) -> bool;
}

pub(crate) struct GSIServer {
    game: Game,
    trusted_token: String,
    pub(crate) http: Server,
    pub(crate) listeners: Vec<Listener>
}

impl GSIServer {
    pub(crate) fn new(game: Game, address: &str, trusted_token: &str) -> Self {
        Self {
            game,
            trusted_token: trusted_token.to_string(),
            http: Server::http(address).unwrap(),
            listeners: Vec::new(),
        }
    }

    pub(crate) fn handle_auth(&self, gsi_request: &SafeRequest) -> Result<(), &str> {
        if gsi_request.authenticate(self.trusted_token.as_ref()) {
            Ok(())
        } else {
            Err("Authentication not passed")
        }
    }

    pub(crate) fn parse_request(&self, request: &mut Request) -> Result<SafeRequest, &str> {
        if get_content_type(&request) != "application/json" {
            return Err("Not a application/json request");
        }

        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();

        let json_request = Arc::new(match self.game {
            Game::Dota => serde_json::from_str::<dota::requests::DotaRequest>(content.as_str()).unwrap(),
            Game::Cs => todo!("Not Implemented yet"),
        });
        Ok(json_request)
    }

    pub(crate) fn respond_with_message(&self, request: Request, message: &str) {
        let response = Response::from_string(message);
        let _ = request.respond(response);
    }
}