extern crate plist;
use std::{error::Error, collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Image {
    color: u32,
    rate: u32,
}

fn main() -> Result<(), Box <dyn Error>> {

    let ps:plist::Value = plist::from_file("IDCLabelInfo.xml").unwrap();

    let root = ps.as_dictionary().unwrap();
    for (&key, &val) in root {
        println!("{key}");
    }

    Ok(())
}
fn _main_0() -> Result<(), Box <dyn Error>>{
    // let li = plist::Value::from_file("IDCLabelInfo.xml")?;
    // for key in li.as_dictionary().unwrap().keys() {
    //     println!("{key}");
    // }

    // let image = Image {color:1, rate:2};
    // let mut images: HashMap<String, Image> = HashMap::new();

    
    // plist::to_file_xml("image0", &image)?;
    // images.insert("H0".to_string(), image);

    // plist::to_file_xml("images", &images)?;
    // // println!("Hello, world!");

    // let mut images2: HashMap<String, Image> = plist::from_file("IDCLabelInfo.xml").unwrap();
    // println!("{}", images2.len());
    // println!("{:?}", images2.get("A_03640__20230122_143452.ARW"));
    // for key in images2.keys().into_iter() {
    //     println!("{key}");
    // }
    
    let ps:plist::Value = plist::from_file("IDCLabelInfo.xml").unwrap();
    println!("{}", ps.into_string().unwrap());
    Ok(())

}
