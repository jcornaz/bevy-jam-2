use bevy::{prelude::*, render::texture::ImageSettings};

mod camera;
mod combine;
mod field;

fn main() {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_plugin(field::Plugin::default())
        .add_plugin(camera::Plugin::default())
        .add_plugin(combine::Plugin::default())
        .run();
}
