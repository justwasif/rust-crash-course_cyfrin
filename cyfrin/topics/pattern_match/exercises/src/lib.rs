pub fn num_to_string(num: u32) -> String {
    
    match num{   
        0=>return "zero".to_string(),
        1=>return "one".to_string(),
        2=>return "two".to_string(),
        3=>return "three".to_string(),
        _=>return "other".to_string(),
    }
}

pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
    todo!();
}
