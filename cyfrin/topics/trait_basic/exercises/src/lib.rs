pub trait Tester {
    fn test(&self, file_path: &str) -> String;
    
}


pub struct Foundry {
    pub version: String,
}

pub struct Cargo {
    pub version: String,
}

pub fn test(tester:&impl Tester, file_path: &str) -> String {
    tester.test(file_path)
}

impl Tester for Foundry{
    fn test(&self, file_path: &str) -> String{
        return "forge test hello.sol".to_string();
    }

}
impl Tester for Cargo{
    fn test(&self,file_path:&str)->String{
        return "cargo test hello.rs".to_string();
    }
}