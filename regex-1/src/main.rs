use regex::Regex;

fn main() {
    let r0 = Regex::new(r"(\d{8})/(\d{6})__(\d{3,5})__(.+)\.([^.]+)").expect("R0 pattern");
    let s0 = "1234/20201201/112303__32123__A6400.1.ARW";
    if let Some(captures) = r0.captures(s0) {
        println!("matches");
        println!(
            "{} {} {} {} {}",
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str(),
            captures.get(3).unwrap().as_str(),
            captures.get(4).unwrap().as_str(),
            captures.get(5).unwrap().as_str(),
        )
    }
    println!("Hello, world!");
}
