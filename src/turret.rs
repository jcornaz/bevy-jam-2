use std::time::Duration;

use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{combine::Harvester, enemy::Enemy, mouse::Cursor, Moving};

#[derive(Debug, Default)]
struct AssetTable {
    turret: Handle<TextureAtlas>,
    bullet: Handle<TextureAtlas>,
}

#[derive(Debug, Clone, Copy, Default, Component)]
struct Turret;

#[derive(Debug, Clone, Component)]
struct Bullet {
    timer: Timer,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(10), false),
        }
    }
}

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .add_startup_system_to_stage(StartupStage::PreStartup, Self::load_assets)
            .add_startup_system(Self::spawn_turret)
            .add_system_to_stage(CoreStage::PreUpdate, Self::aim)
            .add_system(Self::shoot.with_run_criteria(Self::should_shoot))
            .add_system(Self::despawn_bullets)
            .add_system(Self::kill_enemy);
    }
}

impl Plugin {
    fn should_shoot(input: Res<Input<MouseButton>>) -> ShouldRun {
        if input.just_pressed(MouseButton::Left) {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }

    fn shoot(
        mut commands: Commands,
        turrets: Query<&Transform, With<Turret>>,
        assets: Res<AssetTable>,
    ) {
        for turret_transform in &turrets {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: assets.bullet.clone(),
                    transform: *turret_transform,
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Moving { speed: 10.0 })
                .insert(Bullet::default())
                .insert(Name::from("Bullet"));
        }
    }

    fn aim(
        cursor: Res<Cursor>,
        mut turrets: Query<&mut Transform, (With<Turret>, Without<Harvester>)>,
        combines: Query<&Transform, With<Harvester>>,
    ) {
        for mut turret_transform in &mut turrets {
            if let Ok(combine_transform) = combines.get_single() {
                turret_transform.translation = combine_transform.translation + Vec3::Z;
            }

            let direction =
                match (**cursor - turret_transform.translation.truncate()).try_normalize() {
                    Some(d) => d,
                    None => continue,
                };

            turret_transform.rotation =
                Quat::from_axis_angle(Vec3::Z, Vec2::X.angle_between(direction));
        }
    }

    fn kill_enemy(
        mut commands: Commands,
        bullets: Query<&GlobalTransform, With<Bullet>>,
        enemies: Query<(Entity, &GlobalTransform), With<Enemy>>,
    ) {
        for bullet in &bullets {
            for (enemy_entity, enemy) in &enemies {
                let dist_squared = (bullet.translation().truncate()
                    - enemy.translation().truncate())
                .length_squared();
                if dist_squared < 0.3 {
                    commands.entity(enemy_entity).despawn_recursive();
                }
            }
        }
    }

    fn despawn_bullets(
        mut commands: Commands,
        mut bullets: Query<(Entity, &mut Bullet)>,
        time: Res<Time>,
    ) {
        for (entity, mut bullet) in &mut bullets {
            bullet.timer.tick(time.delta());
            if bullet.timer.finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    fn spawn_turret(mut commands: Commands, assets: Res<AssetTable>) {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: assets.turret.clone(),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Turret)
            .insert(Name::from("Turret"));
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
