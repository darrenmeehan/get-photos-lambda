#[macro_use]
use lambda::lambda;
extern crate tokio_core;

use serde_json::Value;
use get_photos_lambda::{get_photos, setup_logging};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(event: Value) -> Result<Value, Error> {
    setup_logging();
    let photos = get_photos().await;
    //
    // for photo in photos {
    //     println!("{}", photo);
    // }
    Ok(event)
}
