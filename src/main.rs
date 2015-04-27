#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(convert)]  // as str
extern crate serde;
use serde::json::{self, Value};
// io
use std::io::prelude::*;
use std::fs::File;

//{"name":"Keeley Bosco",
//"email":"katlyn@jenkinsmaggio.net",
//"city":"Lake Gladysberg",
//"mac":"08:fd:0b:cd:77:f7",
//"timestamp":"2015-04-25 13:57:36 +0700",
//"creditcard":"1228-1221-1221-1431"},
#[derive(Serialize, Deserialize,Debug)]
struct CreditCard {
       name: String,
       email: Option<String>, // Option<str>  Option<'str>
       city: Option<String>,
       mac: String,
       timestamp: String,
       creditcard: Option<String>
}



fn main() {
    let mut json_string = String::new();
    let mut file = File::open("data.json").unwrap();
    file.read_to_string(&mut json_string).unwrap();

    let data: Value = json::from_str(json_string.as_str()).unwrap();
    let vec = data.as_array().unwrap();

    println!("Name, Credit Card");
    // why to_vec here?
    for credit_card in vec.to_vec() {
        let card: CreditCard = json::from_value(credit_card).unwrap();
        if card.creditcard.is_some() {
           println!("{}, {}", card.name, card.creditcard.unwrap());
        }
    }
}
