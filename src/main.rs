use std::env;
use dotenvy::dotenv;


mod config;
mod error;
mod amo;

fn main() {
    dotenv().expect("dotenv init failed");

    println!("ENV: {:#?}", env::vars());
}
