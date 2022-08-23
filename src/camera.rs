use bevy::prelude::*;

use crate::{combine::Harvester, field::Field};

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(Self::spawn)
            .add_system(Self::update_pos);
    }
}

impl Plugin {
    fn spawn(mut commands: Commands) {
        let mut camera = Camera2dBundle::default();
        camera.transform.scale = Vec3::new(0.02, 0.02, 1.0);
        commands.spawn_bundle(camera);
    }

    fn update_pos(
        field: Res<Field>,
        mut cameras: Query<&mut Transform, (With<Camera>, Without<Harvester>)>,
        combine: Query<&Transform, With<Harvester>>,
    ) {
        for mut cam in &mut cameras {
            cam.translation = combine
                .get_single()
                .ok()
                .map(|t| t.translation.truncate())
                .unwrap_or_else(|| field.center().as_vec2())
                .extend(100.0);
        }
    }
}
