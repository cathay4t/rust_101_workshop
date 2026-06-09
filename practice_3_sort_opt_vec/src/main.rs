// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

#[derive(Debug)]
struct Opt<T>(Option<T>);

impl<T: PartialEq> PartialEq for Opt<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Eq for Opt<T> {}

impl<T: Ord> PartialOrd for Opt<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.0, &other.0) {
            (Some(s), Some(o)) => Some(s.cmp(o)),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => Some(Ordering::Equal),
        }
    }
}

impl<T: Ord> Ord for Opt<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut data: Vec<Opt<u8>> = vec![
        Opt(Some(9u8)),
        Opt(None),
        Opt(Some(1)),
        Opt(Some(7)),
        Opt(Some(2)),
        Opt(Some(8)),
    ];
    data.sort_unstable();
    println!("{:?}", data);
}
