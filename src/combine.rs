use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    despawn::despawn,
    field::{Cell, Field, Position},
    GameState,
};

#[derive(Debug, Clone, Copy, Component)]
pub struct Harvester;

pub struct Harvested;

#[derive(Debug, Clone, Component)]
struct Movement {
    direction: IVec2,
    control: Option<IVec2>,
    timer: Timer,
}

const SPEED: f64 = 1.5;

impl Movement {
    fn new(direction: IVec2) -> Self {
        Self {
            direction,
            control: None,
            timer: Timer::new(Duration::from_secs(1).div_f64(SPEED), true),
        }
    }

    fn world_coord(&self, pos: Position) -> Vec2 {
        pos.as_vec2() + (self.direction.as_vec2() * (self.timer.elapsed_secs() * SPEED as f32))
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
        app.add_event::<Harvested>()
            .add_enter_system(GameState::Ready, despawn::<Harvester>)
            .add_enter_system(GameState::Ready, Self::spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(Self::control)
                    .with_system(Self::reverse_in_front_of_barrier)
                    .with_system(Self::movement)
                    .with_system(Self::harvest)
                    .with_system(Self::rotate_sprite)
                    .into(),
            );
    }
}

impl Plugin {
    fn harvest(
        field: ResMut<Field>,
        combine: Query<&Transform, With<Harvester>>,
        mut cells: Query<&mut Cell>,
        mut events: EventWriter<Harvested>,
    ) {
        for position in combine
            .iter()
            .filter_map(|&t| field.get_at(t.translation.truncate()))
        {
            if let Ok(mut cell) = cells.get_mut(position) {
                if let Cell::Crop { level } = *cell {
                    for _ in 0..level {
                        events.send(Harvested);
                    }
                }
                *cell = Cell::Harvested;
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

            movement.control = Some(asked);
        }
    }

    fn reverse_in_front_of_barrier(
        mut combine: Query<(&mut Movement, &Position)>,
        field: Res<Field>,
    ) {
        for (mut movement, position) in &mut combine {
            if (position.x == 0 && movement.direction.x < 0)
                || (position.x == field.width as i32 - 1 && movement.direction.x > 0)
            {
                movement.direction.x = -movement.direction.x;
            }
            if (position.y == 0 && movement.direction.y < 0)
                || (position.y == field.height as i32 - 1 && movement.direction.y > 0)
            {
                movement.direction.y = -movement.direction.y;
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
            asset_server.load("sprites/combine.png"),
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
