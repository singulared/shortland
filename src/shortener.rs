use async_trait::async_trait;
use harsh::{Harsh, HarshBuilder};

#[async_trait]
pub trait Shortner {    
    async fn decode<'a>(&self, url: &'a str) -> u64;
    async fn encode(&self, id: u64) -> String;
}

pub struct HashIds {
    convertor: Harsh
}

// impl Clone for HashIds {
    // fn clone(&self) -> Self {
        // dbg!("++++++++++++++++++++");
        // HashIds {
            // convertor: self.convertor.clone()
        // }
    // }
// }

impl HashIds {
    pub fn new(_salt: Option<String>) -> Self {
        let convertor = HarshBuilder::new().build().unwrap();
        Self {
            convertor
        }
    }
}

#[async_trait]
impl Shortner for HashIds {
    async fn decode<'a>(&self, url: &'a str) -> u64 {
        let a = self.convertor.decode(url);
        dbg!(&a);
        a.unwrap()[0]
    }

    async fn encode(&self, id: u64) -> String {
        self.convertor.encode(&[id])
    }
}
