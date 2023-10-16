use tests_demo::guess::Guess;

mod common;

#[test]
fn test_guess_new() {
    common::setup();
    Guess::new(5);
}

#[test]
#[should_panic]
fn test_guess_new_panic() {
    Guess::new(-1);
}