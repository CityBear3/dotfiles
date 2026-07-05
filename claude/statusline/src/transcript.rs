use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenSums {
    pub input: u64,
    pub output: u64,
    pub cache_5m: u64,
    pub cache_1h: u64,
    pub cache_read: u64,
}

impl TokenSums {
    pub fn add(&mut self, o: &TokenSums) {
        self.input += o.input;
        self.output += o.output;
        self.cache_5m += o.cache_5m;
        self.cache_1h += o.cache_1h;
        self.cache_read += o.cache_read;
    }
}
