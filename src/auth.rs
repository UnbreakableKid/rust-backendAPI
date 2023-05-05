use base64::{engine::general_purpose, Engine as _};
use rocket::request::{FromRequest, Outcome, Request};

use rocket::http::Status;

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: String) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        return Self::from_base64_encoded(split[1]);
    }

    fn from_base64_encoded(encoded: &str) -> Option<BasicAuth> {
        let decoded = general_purpose::STANDARD.decode(encoded).ok()?;
        let decoded = String::from_utf8(decoded).ok()?;
        let split = decoded.split(":").collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        Some(BasicAuth {
            username: split[0].to_string(),
            password: split[1].to_string(),
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Some(auth) = BasicAuth::from_authorization_header(auth_header.to_string()) {
                return Outcome::Success(auth);
            }
        }

        return Outcome::Failure((Status::Unauthorized, ()));
    }
}
