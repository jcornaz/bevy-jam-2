#![deny(future_incompatible, unsafe_code)]
#![warn(nonstandard_style, rust_2018_idioms)]

use bevy::prelude::*;

#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugin(WorldInspectorPlugin::default());

    app.run();
}
