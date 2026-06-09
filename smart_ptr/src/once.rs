// SPDX-License-Identifier: Apache-2.0

use std::{cell::OnceCell, collections::HashMap, sync::OnceLock};

static GLOBAL_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

fn main() {
    let local_num: OnceCell<u32> = OnceCell::new();
    println!("{}", local_num.get_or_init(||3));

    println!(
        "{:?}",
        GLOBAL_MAP.get_or_init(|| {
            let mut ret = HashMap::new();
            ret.insert("foo1".into(), "bar1".into());
            ret.insert("foo2".into(), "bar2".into());
            ret
        })
    );
}
