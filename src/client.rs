use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::error::Error;

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
