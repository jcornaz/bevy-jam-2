use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, Default, Deref, DerefMut)]
pub struct Velocity(pub(crate) Vec2);

pub fn systems() -> SystemSet {
    SystemSet::new().with_system(movement)
}

fn movement(time: Res<Time>, mut movings: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, &velocity) in &mut movings {
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    }
}
