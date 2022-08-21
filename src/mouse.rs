use bevy::{prelude::*, render::camera::RenderTarget};

#[derive(Debug, Clone, Default, Deref)]
pub struct Cursor(Vec2);

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Cursor>()
            .add_system_to_stage(CoreStage::First, Self::track_cursor);
    }
}

impl Plugin {
    fn track_cursor(
        mut cursor: ResMut<Cursor>,
        windows: Res<Windows>,
        cameras: Query<(&Camera, &GlobalTransform)>,
    ) {
        let (camera, camera_transform) = match cameras.get_single() {
            Ok(cam) => cam,
            Err(_) => return,
        };

        let window = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };

        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            cursor.0 = world_pos.truncate();
        }
    }
}
