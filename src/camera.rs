use bevy::prelude::*;

use crate::field::Field;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(Self::spawn);
    }
}

impl Plugin {
    fn spawn(mut commands: Commands, field: Res<Field>) {
        let mut camera = Camera2dBundle::default();
        camera.transform.translation.x = field.width as f32 / 2.0;
        camera.transform.translation.y = field.height as f32 / 2.0;
        camera.transform.scale = Vec3::new(0.01, 0.01, 1.0);
        commands.spawn_bundle(camera);
    }
}
