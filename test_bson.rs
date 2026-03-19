use serde::{Deserialize, Serialize};
use breprs::serialization::bson::{to_bson, from_bson};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestStruct {
    name: String,
    age: u32,
}

fn main() {
    let test = TestStruct {
        name: "Test".to_string(),
        age: 42,
    };
    
    println!("Original: {:?}", test);
    
    let bson_bytes = to_bson(&test).unwrap();
    println!("BSON bytes: {:?}", bson_bytes);
    
    let deserialized: TestStruct = from_bson(&bson_bytes).unwrap();
    println!("Deserialized: {:?}", deserialized);
    
    assert_eq!(test, deserialized);
    println!("Test passed!");
}