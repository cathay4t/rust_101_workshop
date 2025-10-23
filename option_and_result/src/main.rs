// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone)]
struct AbcError {
    kind: ErrorKind,
    msg: String,
}

impl std::fmt::Display for AbcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.kind, self.msg)
    }
}

impl std::error::Error for AbcError {}

struct DataBank {
    numbers: Option<Vec<u8>>,
}

fn count_odd(data: &[u16]) -> usize {
    data.iter().filter(|i| *i % 2 == 1).count()
}

fn main() -> Result<(), AbcError> {
    let a: [u16; 4] = [1u16, 2, 3, 4];

    let count = count_odd(&a);

    println!(
        "Got {} odd number{} in {:?}",
        count,
        if count > 1 { "s" } else { "" },
        a
    );

    Ok(())
}
