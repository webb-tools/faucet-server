#![feature(decl_macro)]

#[macro_use] 
extern crate rocket;

use sled_extensions::DbExt;

mod sled;

fn main() {
    let db = sled_extensions::Config::default()
        .path("./sled_data")
        .open()
        .expect("Failed to open sled db");
        
    rocket::ignite()
        .manage(sled::Database {
            faucets: db
                .open_bincode_tree("users")
                .expect("failed to open user tree"),
        })
        .mount("/api/", routes![sled::post_faucet])
        .launch();
}