extern crate rusoto_dynamodb;
extern crate log;
extern crate simple_logger;

use lambda::lambda;
use serde_json::Value;
use serde_json::json;
use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, QueryInput, ScanInput, ScanOutput, ScanError};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::result::IntoIter;
use std::iter::Map;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn setup_logging() {
    let logger = simple_logger::init();
    match logger {
        Ok(logger) => println!("Logger has been setup successfully"),
        Err(error) => println!("Something went wrongs"),
    };
}

pub async fn get_photos() {
    setup_logging();
    let client = DynamoDbClient::new(Region::EuWest1);
    let scan_input = ScanInput {
        table_name: "photos".to_string(),
        ..Default::default()
    };
    match client.scan(scan_input).await {
        Ok(output) => {
            match output.items {
                Some(photos) => {
                    println!("Photos in database:");
                    for single_photo in photos {
                        // println!("{}", single_photo)
                        println!("Just got a photo.. Details inbound!");
                    }
                }
                None => println!("No photos found.."),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    // let photos = response
    //     .into_iter()
    //     .map(|s| s)
    //     .filter_map(Result::ok)
    //     .collect();
    // println!("Results: {}", photos);
    // // // FIXME Need to read https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
    // // let photos = match photos {
    // //     Ok(photos) =>photos,
    // //     Err(error) => panic!("Something has gone wrong.."),
    // // };
    // photos
}
