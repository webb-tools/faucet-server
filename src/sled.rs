use rocket::State;
use rocket_contrib::json::Json;
use rocket::{http::Status, response::Responder};

use serde::{Deserialize, Serialize};
use sled_extensions::bincode::Tree;
use std::option::Option::None;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("sled db error")]
    SledError(#[from] sled_extensions::Error),
    #[error("resource not found")]
    NotFound,
}

impl<'a> Responder<'a> for ServerError {
    fn respond_to(self, _: &rocket::Request) -> Result<rocket::Response<'a>, Status> {
        match self {
            Self::SledError(_) => Err(Status::InternalServerError),
            Self::NotFound => Err(Status::NotFound),
        }
    }
}

type EndpointResult<T> = Result<T, ServerError>;

pub struct Database {
    pub faucets: Tree<Faucet>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Faucet {
    id: String,
    account: String,
    amount: String,
}

#[post("/faucets", data = "<faucet>")]
pub fn post_faucet(db: State<Database>, faucet: Json<Faucet>) -> EndpointResult<Json<Faucet>> {
    // let exist_faucet = Json(db.faucets.get(faucet.account.as_bytes()));
    // Ok(exist_faucet);
    db.faucets
        .insert(faucet.account.as_bytes(), faucet.clone())
        .unwrap();
    Ok(Json(faucet.0))
}
