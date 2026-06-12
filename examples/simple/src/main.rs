use mistserver_rs::{MistClient, MistClientBuilder};

#[tokio::main]
async fn main() {
    let mist_client = MistClientBuilder::new("http://localhost:4242/api", None);

    println!("Hello, world!");
}
