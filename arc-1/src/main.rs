use std::sync::Arc;

struct Name {
    name0: String,
    name1: String,
}

fn main() {

    let mut names = Arc::new(Name {
        name0: "0".to_string(),
        name1: "1".to_string()
    });

    let name_ref = Arc::get_mut(&mut names);
    match name_ref {
        Some(inner) => {
            inner.name0 = "append0".to_string();
        },
        None => {
            println!("无法获取");
        }
    }

    println!("Hello, world! {}", names.name0);
}
