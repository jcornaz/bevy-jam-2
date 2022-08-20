use bevy::prelude::*;

use crate::field::Field;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn);
    }
}

impl Plugin {
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

        #[cfg_attr(not(feature = "inspector"), allow(unused))]
        let mut entity = commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_translation(field.center().into_translation(1.0)),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            ..Default::default()
        });

        #[cfg(feature = "inspector")]
        entity.insert(Name::from("Combine"));
    }
}
