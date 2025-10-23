// SPDX-License-Identifier: Apache-2.0

fn main() {
    let mut data = Some(vec![9u8, 1, 7, 2, 8]);
    data.as_mut().map(|d| d.sort_unstable());
    println!("{:?}", data);
}
