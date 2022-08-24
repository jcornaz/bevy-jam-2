use std::f32::consts::PI;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn::despawn, field::Field, GameState};

#[derive(Debug, Default, Component)]
struct Barrier;

#[derive(Default)]
struct AssetTable {
    border: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .add_startup_system(Self::load_assets)
            .add_enter_system(GameState::Ready, despawn::<Barrier>)
            .add_enter_system(GameState::Ready, Self::spawn);
    }
}

impl Plugin {
    fn spawn(mut commands: Commands, field: Res<Field>, assets: Res<AssetTable>) {
        const Z: f32 = 0.0;
        for x in 0..field.width {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: assets.border.clone(),
                    transform: Transform::from_xyz(x as f32, field.height as f32, Z),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Barrier);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: assets.border.clone(),
                    transform: Transform::from_xyz(x as f32, -1.0, Z),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        flip_y: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Barrier);
        }
        for y in 0..field.height {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: assets.border.clone(),
                    transform: Transform::from_xyz(-1.0, y as f32, Z)
                        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 2.0)),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Barrier);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: assets.border.clone(),
                    transform: Transform::from_xyz(field.width as f32, y as f32, Z)
                        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 2.0)),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        flip_y: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Barrier);
        }
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: assets.border.clone(),
                transform: Transform::from_xyz(-1.0, field.height as f32, Z),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    index: 1,
                    flip_x: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Barrier);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: assets.border.clone(),
                transform: Transform::from_xyz(field.width as f32, field.height as f32, Z),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    index: 1,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Barrier);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: assets.border.clone(),
                transform: Transform::from_xyz(-1.0, -1.0, Z),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    index: 1,
                    flip_x: true,
                    flip_y: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Barrier);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: assets.border.clone(),
                transform: Transform::from_xyz(field.width as f32, -1.0, Z),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    index: 1,
                    flip_y: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Barrier);
    }

    fn load_assets(
        asset_server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<AssetTable>,
    ) {
        assets.border = textures.add(TextureAtlas::from_grid(
            asset_server.load("sprites/barrier.png"),
            Vec2::splat(32.0),
            2,
            1,
        ));
    }
}
