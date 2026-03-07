
pub fn eq(x:char,y:char) ->bool{
    if x==y{
        return true;
    }else {
        return false;
    }
}

pub fn add(x:f32,y:f32,z:f32)->f32 {
    let sum:f32=x+y+z;
    return sum;
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    let sum=x as f32 + y as f32 + z; 
    return sum;
}
