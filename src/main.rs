//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]
#![feature(convert)]  // as str
extern crate serde;
use serde::json::{self, Value};
// io
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut json_string = String::new();
    let mut file = File::open("data.json").unwrap();
    file.read_to_string(&mut json_string).unwrap();
    let data: Value = json::from_str(json_string.as_str()).unwrap();

    let vec = data.as_array().unwrap();

    println!("Name, Credit Card");
    for credit_card in vec {
        let card =  credit_card.as_object().unwrap();
        if !card.get("creditcard").unwrap().is_null() {
        println!("{}, {}",
                 card.get("name").unwrap().as_string().unwrap(),
                 card.get("creditcard").unwrap().as_string().unwrap(),
        );
        }
    }
    // bar: baz (string)
    // foo: 13 (u64)
}
