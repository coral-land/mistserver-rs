use mistserver_rs::MistClientBuilder;

#[tokio::main]
async fn main() {
    let mist_client = MistClientBuilder::new("http://localhost:4242/api");
    println!("Hello, world!");
}
