// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Abc {
    foo: String,
    bar: u8,
}

fn main() {
    let data: Vec<Abc> = serde_yaml::from_str(
        r#"---
        - foo: abc
          bar: 100
        - foo: abd
          bar: 10"#,
    )
    .unwrap();
    println!("Got{:?}",  data);

    println!("YAML output:\n{}", serde_yaml::to_string(&data).unwrap());
}
