use bevy::prelude::*;

use crate::combine::Harvester;

#[derive(Debug, Default)]
struct AssetTable {
    turret: Handle<TextureAtlas>,
    bullet: Handle<TextureAtlas>,
}

/// Marker component, indicating the combine is equiped with a turret
#[derive(Debug, Clone, Copy, Default, Component)]
struct HasTurret;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .add_startup_system(Self::load_assets)
            .add_system_to_stage(CoreStage::PreUpdate, Self::spawn_turret);
    }
}

impl Plugin {
    fn spawn_turret(
        mut commands: Commands,
        combines: Query<Entity, (With<Harvester>, Without<HasTurret>)>,
        assets: Res<AssetTable>,
    ) {
        for combine_entity in &combines {
            commands
                .entity(combine_entity)
                .insert(HasTurret)
                .with_children(|children| {
                    children
                        .spawn_bundle(SpriteSheetBundle {
                            transform: Transform::from_translation(Vec3::Z),
                            texture_atlas: assets.turret.clone(),
                            sprite: TextureAtlasSprite {
                                custom_size: Some(Vec2::ONE),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Name::from("Turret"));
                });
        }
    }

    fn load_assets(
        mut table: ResMut<AssetTable>,
        server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
    ) {
        table.turret = textures.add(TextureAtlas::from_grid(
            server.load("turret.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
        table.bullet = textures.add(TextureAtlas::from_grid(
            server.load("bullet.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
    }
}
