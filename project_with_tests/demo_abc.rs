// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Default)]
pub struct DemoStruct {
    data: String,
}

impl DemoStruct {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &str {
        self.data.as_str()
    }

    pub fn delete(&self) {
        println!("Deleting {}", self.data);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data() {
        let foo = DemoStruct::new("abc".into());

        assert_eq!(foo.data(), "abc");
    }
}
