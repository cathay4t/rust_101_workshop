// SPDX-License-Identifier: Apache-2.0

fn main() {
    let org = vec![1u8,2,3,4];
    let mut a = org;


    let item0 = &a[0];

    // uncommon this line will cause compile failure
    // a.push(5);

    println!("{}", item0);

    a.push(5);

    println!("{:?}", a);
}
