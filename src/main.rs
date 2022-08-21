use bevy::{prelude::*, render::texture::ImageSettings};

mod camera;
mod combine;
mod enemy;
mod field;
mod mouse;
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
        .add_system(apply_velocity)
        .run();
}

#[derive(Debug, Clone, Copy, Component, Default, Deref, DerefMut)]
struct Moving {
    speed: f32,
}

fn apply_velocity(time: Res<Time>, mut movings: Query<(&mut Transform, &Moving)>) {
    for (mut transform, &moving) in &mut movings {
        let rotation = transform.rotation;
        transform.translation += rotation * (Vec3::X * moving.speed * time.delta_seconds());
    }
}
