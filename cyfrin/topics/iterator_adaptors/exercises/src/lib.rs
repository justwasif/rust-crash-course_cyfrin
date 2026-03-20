use std::collections::HashMap;

pub fn filter_non_zero(v: Vec<u32>) -> Vec<u32> {
    let mut ret:Vec<u32>=vec![];
    for i in v{
        if i!=0{
            ret.push(i);
        }
    }
    return ret;
}

pub fn to_string(v: Vec<&str>) -> Vec<String> {
    let mut vec_str:Vec<String>=vec![];
    for i in v{
        let a=i.to_string();
        vec_str.push(a);
    }
    return vec_str;
}

pub fn to_hash_map(v: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut h:HashMap<String,u32>=HashMap::new();
    for (i ,k) in v{
        h.insert(i,k);
    }
    return h;
}