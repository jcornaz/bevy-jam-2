use std::time::Duration;

use bevy::prelude::*;

use crate::field::{Cell, Field, Position};

#[derive(Debug, Clone, Copy, Component)]
pub struct Harvester;

#[derive(Debug, Clone, Component)]
struct Movement {
    direction: IVec2,
    control: Option<IVec2>,
    timer: Timer,
}

impl Movement {
    fn new(direction: IVec2) -> Self {
        Self {
            direction,
            control: None,
            timer: Timer::new(Duration::from_secs(1), true),
        }
    }

    fn world_coord(&self, pos: Position) -> Vec2 {
        pos.as_vec2() + (self.direction.as_vec2() * self.timer.elapsed_secs())
    }

    fn update(&mut self, pos: &mut Position, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            **pos += self.direction;
            if let Some(control) = self.control {
                self.direction = control;
                self.control = None;
            }
        }
    }
}

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn)
            .add_system(Self::movement)
            .add_system(Self::control)
            .add_system(Self::harvest)
            .add_system_to_stage(CoreStage::PostUpdate, Self::rotate_sprite);
    }
}

impl Plugin {
    fn harvest(
        field: ResMut<Field>,
        combine: Query<&Transform, With<Harvester>>,
        mut cells: Query<&mut Cell>,
    ) {
        for position in combine
            .iter()
            .filter_map(|&t| field.get_at(t.translation.truncate()))
        {
            if let Ok(mut cell) = cells.get_mut(position) {
                cell.harvest();
            }
        }
    }

    fn control(input: Res<Input<KeyCode>>, mut combine: Query<&mut Movement>) {
        for mut movement in combine.iter_mut() {
            let asked = if input.pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
                IVec2::Y
            } else if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
                -IVec2::Y
            } else if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
                IVec2::X
            } else if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
                -IVec2::X
            } else {
                return;
            };

            if asked != -movement.direction {
                movement.control = Some(asked);
            }
        }
    }

    fn movement(
        time: Res<Time>,
        mut combine: Query<(&mut Transform, &mut Movement, &mut Position)>,
    ) {
        for (mut transform, mut movement, mut pos) in combine.iter_mut() {
            movement.update(&mut pos, time.delta());
            transform.translation = movement.world_coord(*pos).extend(transform.translation.z);
        }
    }

    fn rotate_sprite(mut combines: Query<(&mut Transform, &Movement)>) {
        for (mut transform, movement) in combines.iter_mut() {
            let direction = movement.direction.as_vec2();
            transform.rotation = Quat::from_axis_angle(Vec3::Z, Vec2::X.angle_between(direction))
        }
    }

    fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
        field: Res<Field>,
    ) {
        let texture_atlas = textures.add(TextureAtlas::from_grid(
            asset_server.load("combine.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));

        let position = field.center();
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas,
                transform: Transform::from_translation(position.as_vec2().extend(1.0)),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(position)
            .insert(Harvester)
            .insert(Movement::new(IVec2::X))
            .insert(Name::from("Combine"));
    }
}
