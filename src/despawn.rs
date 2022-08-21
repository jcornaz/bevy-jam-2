use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct DespawnTimer(Timer);

pub fn systems() -> SystemSet {
    SystemSet::new().with_system(after_timeout)
}

fn after_timeout(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in &mut bullets {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl DespawnTimer {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, false))
    }
}
