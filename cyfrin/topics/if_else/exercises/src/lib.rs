pub fn min(x: i32, y: i32) -> i32 {
    let k=if x>y{
        y
    }else {
        x
    };
    return k;
}

pub fn max(x: i32, y: i32) -> i32 {
    let k=if x>y{
        x
    }else {
        y
    };
    return k;
}

pub fn sign(x: i32) -> i32 {
    let k=if x<0{
        -1
    }else  {
        1
    };
    return k;
}
