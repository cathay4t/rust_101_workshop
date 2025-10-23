// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

struct DataBank<'a> {
    name: &'a str,
    data: HashMap<String, String>,
}

impl<'a> DataBank<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
}

fn get_bank_data<'a>(bank: 'a DataBank, key: &str) -> Option<&'a str> {
    bank.data.get(key).map(|s| s.as_str())
}

fn main() {
    let mut data = HashMap::new();
    data.insert("key1".to_string(), "value1".to_string());
    let bank = DataBank { name: "foo", data };

    println!("{}: {:?}", bank.name, get_bank_data(&bank, "key1"));
    println!("{}: {:?}", bank.name, bank.get("key1"));
}
