// SPDX-License-Identifier: Apache-2.0

fn main() {
    let mut a = vec![1u8, 2, 3, 4];
    let item0 = &a[0];

    a.push(5);

    println!("{}", item0);
    println!("{:?}", a);
}
