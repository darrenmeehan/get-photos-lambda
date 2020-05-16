#[macro_use]
extern crate log;
extern crate simple_logger;
use lambda::lambda;
use serde_json::Value;
use serde_json::json;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(event: Value) -> Result<Value, Error> {
    let logger = simple_logger::init();
    match logger {
        Ok(logger) => println!("Logger has been setup successfully"),
        Err(error) => println!("Something went wrongs"),
    };

    let message = json!("Hello from Lambda");

    Ok(message)
}
