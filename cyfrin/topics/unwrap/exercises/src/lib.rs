pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let c:u32=a.parse().expect("Failed to parse variable");
    let d:u32=b.parse().expect("Failed to parse variable");
    return c+d;   
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    let a=x.unwrap();
    let b=y.unwrap();
    return a+b;
}
