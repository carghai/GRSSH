use crate::host::host_main;

mod command;
mod db;
mod host;
pub mod ram_var;

const SHELL: &str = "zsh";
const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";

const NAME: &str = "test";

fn main() {
    db::client().expect("please check your redis connection");
    println!("starting host version");
    host_main();
}