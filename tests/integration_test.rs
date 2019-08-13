#[macro_use]
extern crate lazy_static;
extern crate mockito;
extern crate moe;

use mockito::mock;
use moe::{Client, Doc, Result};
use std::fs;

lazy_static! {
    static ref CLIENT: Client = {
        let mut client = Client::new();
        client.base_uri = mockito::server_url();
        client
    };
}

#[test]
fn test_hataraku_saibou_07() -> Result<()> {
    let _m = mock("POST", "/search")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/fixtures/hataraku_saibou_07.response.http")
        .create();

    let image = fs::read("tests/fixtures/hataraku_saibou_07.jpg").unwrap();
    let search_response = CLIENT.search(image)?;
    assert!(!search_response.docs.is_empty());
    let first_doc: &Doc = &search_response.docs[0];
    assert_eq!("Hataraku Saibou", first_doc.title_romaji);
    Ok(())
}

#[test]
fn test_me() -> Result<()> {
    let _m = mock("GET", "/me")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/fixtures/me.response.http")
        .create();

    let me = CLIENT.me()?;
    assert_eq!(None, me.user_id);
    Ok(())
}
