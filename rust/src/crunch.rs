use godot::classes::rendering_server::ArrayType;
use godot::prelude::*;
use godot::classes::{Node, INode, Image};
use godot::builtin::Array;
use godot::meta::ArrayElement;
use crunch::*;
use std::vec;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct CrunchPacker {
    items: Vec<Item<i64>>,
    base: Base<Node>
}

#[godot_api]
impl CrunchPacker {
    fn init(base: Base<Node>) -> Self {
        godot_print!("HELLO WORLD!");
        Self {
            items: vec![],
            base
        }
    }

    #[func]
    fn add_item(&mut self, id: i64, w: i64, h: i64) {
        self.items.push(Item::new(id, w as usize, h as usize, Rotation::None));
    }

    #[func]
    fn crunch(&mut self) -> Vec<PackedItem<i64>> {

    }
    /*[func]
    fn pack_images(&mut self, images: Array<ArrayType<Image>>) {
        for item in images.iter_shared() {

        }
    }*/

}
