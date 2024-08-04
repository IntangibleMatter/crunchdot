use core::panic;
use crunch::*;
use godot::classes::{AtlasTexture, IRefCounted, Image, RefCounted, Texture2D};
use godot::obj::NewGd;
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
    fn pack(&mut self, max_width: i64, max_height: i64) {
        match crunch::pack_into_po2(2048, self.items.clone()) {
            Ok(PackedItems { w, h, items }) => {}
            Err(_) => {
                panic!("Failed to pack images.")
            }
        }
    }

    #[func]
    fn pack_into_po2(&mut self, max_po2: i64) {}
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct CrunchableItemData {
    #[var]
    path: GString,
    image: Gd<Image>,
    #[var]
    og_rect: Rect2i,
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for CrunchableItemData {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            path: GString::new(),
            image: Image::new_gd(),
            og_rect: Rect2i {
                position: Vector2i { x: 0, y: 0 },
                size: Vector2i { x: 0, y: 0 },
            },
        }
    }
}

#[godot_api]
impl CrunchableItemData {
    #[func]
    fn from_texture(tex: Gd<Texture2D>, trim: bool) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            path: tex.get_path(),
            og_rect: Rect2i {
                position: Vector2i::ZERO,
                size: Vector2i {
                    x: tex.get_width(),
                    y: tex.get_height(),
                },
            },
            image: Self::tex_to_img(tex, trim),
            base,
        })
    }

    fn tex_to_img(tex: Gd<Texture2D>, trim: bool) -> Gd<Image> {
        if trim {
            let opt_img = tex.get_image();

            if opt_img.is_some() {
                let img = opt_img.unwrap();
                let used = img.get_used_rect();

                let mut out_img =
                    Image::create(used.size.x, used.size.y, false, img.get_format()).unwrap();
                out_img.blit_rect(img, used, Vector2i::ZERO);
                out_img
            } else {
                Image::new_gd()
            }
        } else {
            tex.get_image().unwrap()
        }
    }
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct CrunchedItemData {
    #[var]
    path: GString,
    #[var]
    atlastex: Gd<AtlasTexture>,
    #[var]
    atlasrect: Rect2i,
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for CrunchedItemData {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            path: GString::new(),
            atlastex: AtlasTexture::new_gd(),
            atlasrect: Rect2i {
                position: Vector2i::ZERO,
                size: Vector2i::ZERO,
            },
        }
    }
}

#[godot_api]
impl CrunchedItemData {
    #[func]
    fn commmit_texture(&mut self) {
        self.atlastex.take_over_path(self.path.clone());
    }
}
