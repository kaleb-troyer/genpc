
// Class Structure and Collection
#[derive(Debug)]
pub struct Class {
    pub name: &'static str,
    pub hit_die: u8,
}

pub static CLASSES: &[Class] = &[
    Class {
        name: "Fighter",
        hit_die: 10,
    },
    Class {
        name: "Paladin",
        hit_die: 10,
    }
];

// EOF
