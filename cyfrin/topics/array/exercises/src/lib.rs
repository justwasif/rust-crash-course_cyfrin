pub fn zeros() -> [u32; 100] {
    let arr=[0;100];
    return arr
}

pub fn first_3(s: &[u32]) -> &[u32] {
    let first3=&s[..3];
    return first3;
}

pub fn last_3(s: &[u32]) -> &[u32] {
    let n=s.len();
    let last3=&s[n-3..];
    return last3;
}
