#![deny(future_incompatible, unsafe_code)]
#![warn(nonstandard_style, rust_2018_idioms)]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugin(ShapePlugin);

    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugin(WorldInspectorPlugin::default());

    app.run();
}
