use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use sled_extensions::bincode::Tree;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("sled db error")]
    SledError(#[from] sled_extensions::Error),
    #[error("resource not found")]
    NotFound,
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
    db.faucets
        .insert(faucet.account.as_bytes(), faucet.clone())
        .unwrap();
    Ok(Json(faucet.0))
}
