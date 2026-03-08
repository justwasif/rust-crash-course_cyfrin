pub fn hello() -> String {
    let x:String=String::from("Hello Rust");
    return x;
}

pub fn greet(name: &str) -> String {
    let mut he="Hello ".to_string();
    he+=name;
    return he;
}

pub fn append(mut s: String) -> String {
    s+="!";
    return s;
}
