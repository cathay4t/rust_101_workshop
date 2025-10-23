// SPDX-License-Identifier: Apache-2.0

fn main() {
    let mut data = vec![9u8, 1, 7, 2, 8];
    data.sort_unstable();
    println!("{:?}", data);
}
