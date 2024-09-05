use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;
fn main() {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set in .env file");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set in .env file");

    let mut file = File::create("src/env_vars.rs").expect("Unable to create env_vars.rs file");
    writeln!(file, "pub const CLIENT_ID: &str = \"{}\";", client_id).unwrap();
    writeln!(
        file,
        "pub const CLIENT_SECRET: &str = \"{}\";",
        client_secret
    )
    .unwrap();
}
