// SPDX-License-Identifier: Apache-2.0

trait CanFly {
    fn can_fly(&self) -> bool;
}

trait GenerateEta: CanFly {
    fn eta_sec(&self) -> u32 {
        if self.can_fly() { 10 } else { 1000 }
    }
}

impl<T> GenerateEta for T where T: CanFly {}

struct Duck {
    name: String,
}

impl CanFly for Duck {
    fn can_fly(&self) -> bool {
        true
    }
}

struct Human {
    name: String,
    role: String,
}

impl CanFly for Human {
    fn can_fly(&self) -> bool {
        self.role == "pilot"
    }
}

fn main() {
    let duck = Duck {
        name: "duck".into(),
    };
    let dev = Human {
        name: "dev".into(),
        role: "dev".into(),
    };
    let pilot = Human {
        name: "pilot".into(),
        role: "pilot".into(),
    };

    println!("{} ETA {} seconds", duck.name, duck.eta_sec());
    println!("{} ETA {} seconds", dev.name, dev.eta_sec());
    println!("{} ETA {} seconds", pilot.name, pilot.eta_sec());
}
