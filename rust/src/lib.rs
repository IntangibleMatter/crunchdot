use godot::prelude::*;

mod crunch;

struct Crunchdot;

#[gdextension]
unsafe impl ExtensionLibrary for Crunchdot {}
