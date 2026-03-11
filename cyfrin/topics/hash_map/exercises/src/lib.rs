use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut H:HashMap<String,u32>=HashMap::new();
    H.insert(address,amount);
    return H;
}
