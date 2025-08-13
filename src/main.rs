use std::env;
use dotenvy::dotenv;


mod config;
mod error;
mod amo;
mod xlsx;

fn main() {
    dotenv().expect("dotenv init failed");

    println!("ENV: {:#?}", env::vars());
}
