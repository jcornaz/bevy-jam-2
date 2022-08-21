use std::time::Duration;

use bevy::prelude::*;

use crate::field::{Cell, Field, Position};

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
struct Direction(IVec2);

#[derive(Debug, Clone, Copy, Component, Default, Deref, DerefMut)]
struct Control(IVec2);

#[derive(Debug, Clone, Component, Deref, DerefMut)]
struct MovementTimer(Timer);

impl Default for MovementTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(1), true))
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
        combine: Query<&Position, (Changed<Position>, With<Control>)>,
        mut cells: Query<&mut Cell>,
    ) {
        for position in combine.iter().filter_map(|&p| field.get(p)) {
            if let Ok(mut cell) = cells.get_mut(position) {
                cell.harvest();
            }
        }
    }

    fn control(input: Res<Input<KeyCode>>, mut combine: Query<(&mut Control, &Direction)>) {
        for (mut control, &direction) in combine.iter_mut() {
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

            if asked != -*direction {
                **control = asked;
            }
        }
    }

    fn movement(
        time: Res<Time>,
        mut combine: Query<(
            &mut MovementTimer,
            &mut Transform,
            &mut Position,
            &mut Direction,
            &mut Control,
        )>,
    ) {
        for (mut timer, mut transform, mut position, mut direction, mut control) in
            combine.iter_mut()
        {
            timer.tick(time.delta());
            if timer.just_finished() {
                **position += **direction;
                if control.x != 0 || control.y != 0 {
                    **direction = **control;
                    *control = Control::default();
                }
            }
            transform.translation = position.as_vec2().extend(transform.translation.z)
                + (direction.as_vec2() * timer.elapsed_secs()).extend(0.0);
        }
    }

    fn rotate_sprite(mut combines: Query<(&mut Transform, &Direction), Changed<Direction>>) {
        for (mut transform, &direction) in combines.iter_mut() {
            let direction = direction.as_vec2();
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
        let mut entity = commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_translation(position.as_vec2().extend(1.0)),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            ..Default::default()
        });

        entity
            .insert(position)
            .insert(Direction(IVec2::X))
            .insert(Control::default())
            .insert(MovementTimer::default());

        #[cfg(feature = "inspector")]
        entity.insert(Name::from("Combine"));
    }
}
