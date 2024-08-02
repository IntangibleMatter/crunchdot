use crunch::*;
use godot::builtin::Array;
use godot::classes::{IRefCounted, RefCounted};
use godot::meta::ArrayElement;
use godot::prelude::*;
use std::vec;

#[derive(GodotClass)]
#[class(base=RefCounted)]

struct CrunchPacker {
    items: Vec<Item<i64>>,

    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for CrunchPacker {
    fn init(base: Base<RefCounted>) -> Self {
        godot_print!("HELLO WORLD!");
        Self {
            items: vec![],
            base,
        }
    }
}
#[godot_api]
impl CrunchPacker {
    #[func]
    fn add_item(&mut self, id: i64, w: i64, h: i64) {
        self.items
            .push(Item::new(id, w as usize, h as usize, Rotation::None));
    }

    #[func]
    fn crunch(&mut self) {}
}
