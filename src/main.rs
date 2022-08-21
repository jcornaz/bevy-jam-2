use bevy::{prelude::*, render::texture::ImageSettings};

mod camera;
mod combine;
mod despawn;
mod enemy;
mod field;
mod mouse;
mod movement;
mod turret;

fn main() {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_plugin(mouse::Plugin::default())
        .add_plugin(field::Plugin::default())
        .add_plugin(camera::Plugin::default())
        .add_plugin(combine::Plugin::default())
        .add_plugin(enemy::Plugin::default())
        .add_plugin(turret::Plugin::default())
        .add_system_set(movement::systems())
        .add_system_set(despawn::systems())
        .run();
}
