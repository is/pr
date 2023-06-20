extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
    interests: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
enum Color0 {
    C {red:u8, green:u8, blue:u8},
    N {n:i32},
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "op", rename_all="UPPERCASE")]
enum Color1 {
    C { red:u8, green:u8, blue:u8},
    N {n:i32},
}

fn main() {
    let json_str = r#"
        {
            "name": "John",
            "age": 30,
            "is_male": true,
            "interests": ["programming", "reading", "hiking"]
        }
    "#;

    let person: Person = serde_json::from_str(json_str).unwrap();
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Is male: {}", person.is_male);
    println!("Interests: {:?}", person.interests);

    println!("--Person: {}", serde_json::to_string(&person).unwrap());

    let c0 = Color0::N{n: 64};
    println!("--Color0: {}", serde_json::to_string(&c0).unwrap());

    let c1 = Color1::C{ red: 12, green:20, blue:30};
    println!("--Color1: {}", serde_json::to_string(&c1).unwrap());

    let c2: Color1 = serde_json::from_str(
        serde_json::to_string(&c1).unwrap().as_str()).unwrap();
    println!("--Color1: {:?}", c2)
}
