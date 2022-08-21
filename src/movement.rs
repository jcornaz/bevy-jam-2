use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, Default, Deref, DerefMut)]
pub struct Moving {
    pub speed: f32,
}

pub fn systems() -> SystemSet {
    SystemSet::new().with_system(movement)
}

fn movement(time: Res<Time>, mut movings: Query<(&mut Transform, &Moving)>) {
    for (mut transform, &moving) in &mut movings {
        let rotation = transform.rotation;
        transform.translation += rotation * (Vec3::X * moving.speed * time.delta_seconds());
    }
}
