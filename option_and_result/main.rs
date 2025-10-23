// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ErrorKind {
    EmptyData,
}

#[derive(Debug, Clone)]
pub struct AbcError {
    pub kind: ErrorKind,
    pub msg: String,
}

impl std::fmt::Display for AbcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.kind, self.msg)
    }
}

impl std::error::Error for AbcError {}

#[derive(Debug)]
struct DataBank {
    numbers: Option<Vec<u8>>,
}

impl DataBank {
    pub fn new(numbers: Vec<u8>) -> Self {
        Self {
            numbers: Some(numbers),
        }
    }

    pub fn count_odd(&self) -> Option<usize> {
        Some(
            self.numbers
                .as_ref()?
                .iter()
                .filter(|i| *i % 2 == 1)
                .count(),
        )
    }

    pub fn count_odd_v2(&self) -> Option<usize> {
        if let Some(numbers) = self.numbers.as_ref() {
            Some(numbers.iter().filter(|i| *i % 2 == 1).count())
        } else {
            None
        }
    }

    pub fn count_odd_v3(&self) -> Option<usize> {
        self.numbers
            .as_ref()
            .map(|n| n.iter().filter(|i| *i % 2 == 1).count())
    }

    pub fn try_count_odd(&self) -> Result<usize, AbcError> {
        self.count_odd().ok_or(AbcError {
            kind: ErrorKind::EmptyData,
            msg: "Empty bank".into(),
        })
    }
}

fn main() -> Result<(), AbcError> {
    let bank = DataBank::new(vec![1u8, 2, 3, 4]);

    println!("{:?}", bank);

    let count = bank.count_odd();
    println!("{:?}", count);

    let count = bank.count_odd_v2();
    println!("{:?}", count);

    let count = bank.count_odd_v3();
    println!("{:?}", count);

    let count = bank.try_count_odd()?;
    println!("{}", count);

    Ok(())
}
