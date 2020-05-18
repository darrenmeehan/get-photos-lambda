extern crate get_photos_lambda;

// importing common module.
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();
    assert_eq!(get_photos_lambda::add(3, 2), 5);
}

#[test]
fn test_get_photos() {
    common::setup();
    let photos = get_photos_lambda::get_photos();
}
