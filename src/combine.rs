use bevy::prelude::*;

use crate::field::Field;

/// Speed, expressed in tile-per-second
const SPEED: f32 = 1.0;

#[derive(Debug, Clone, Copy, Component)]
struct Direction(Vec2);

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn)
            .add_system(Self::movement);
    }
}

impl Plugin {
    fn movement(time: Res<Time>, mut combine: Query<(&mut Transform, &Direction)>) {
        let (mut transform, direction) = combine.single_mut();
        transform.translation += (direction.0 * (time.delta_seconds() * SPEED)).extend(0.0);
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
            transform: Transform::from_translation(position.into_translation(1.0)),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            ..Default::default()
        });

        entity.insert(Direction(Vec2::X));

        #[cfg(feature = "inspector")]
        entity.insert(Name::from("Combine"));
    }
}
