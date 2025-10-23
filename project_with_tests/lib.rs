// SPDX-License-Identifier: Apache-2.0

/// Demo crate
///
/// This is library front page document.
/// Example code which will be compiled and run by `cargo test`:
///
/// ```rust
/// use libdemo::DemoStruct;
///
/// println!("{}", DemoStruct::default().data());
/// ```
///
/// Example code which will be compiled but not invoked by `cargo test`:
///
/// ```no_run
/// use libdemo::DemoStruct;
///
/// let data = DemoStruct::new("abc".into());
/// data.delete();
/// ```
mod demo_abc;

pub use self::demo_abc::DemoStruct;
