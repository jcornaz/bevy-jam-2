use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    combine::{Harvested, Harvester},
    despawn::{despawn, DespawnTimer},
    enemy::Enemy,
    mouse::Cursor,
    movement::Moving,
    GameState,
};

#[derive(Debug, Default)]
struct AssetTable {
    turret: Handle<TextureAtlas>,
    bullet: Handle<TextureAtlas>,
}

#[derive(Debug, Clone, Component)]
struct Turret {
    cool_down: Timer,
}

impl Default for Turret {
    fn default() -> Self {
        Self {
            cool_down: Timer::new(Duration::ZERO, false),
        }
    }
}

#[derive(Debug, Clone, Default, Component, Deref, DerefMut)]
pub struct Ammo(u32);

#[derive(Debug, Clone, Component, Default)]
struct Bullet;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .init_resource::<Ammo>()
            .add_startup_system(Self::load_assets)
            .add_enter_system(GameState::Playing, despawn::<Turret>)
            .add_enter_system(GameState::Playing, Self::spawn_turret)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                Self::aim.run_in_state(GameState::Playing),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(Self::spawn_bullet.run_if(Self::shoot))
                    .with_system(Self::kill_enemy)
                    .into(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::recharge.run_in_state(GameState::Playing),
            );
    }
}

impl Plugin {
    #[allow(unused)]
    fn log_ammo(ammos: Query<&Ammo>) {
        for ammo in &ammos {
            info!("Ammo: {:?}", **ammo);
        }
    }

    fn recharge(mut harvests: EventReader<Harvested>, mut ammos: Query<&mut Ammo>) {
        const AMMO_PER_CROP_CELL: u32 = 4;
        const MAX_AMMO: u32 = 20;
        let delta_ammo = harvests.iter().count() as u32 * AMMO_PER_CROP_CELL;
        if delta_ammo == 0 {
            return;
        }
        for mut ammo in &mut ammos {
            **ammo = delta_ammo.min(MAX_AMMO);
        }
    }

    fn shoot(
        input: Res<Input<MouseButton>>,
        mut turrets: Query<(&mut Turret, &mut Ammo)>,
        time: Res<Time>,
    ) -> bool {
        let (mut turret, mut ammo) = match turrets.get_single_mut() {
            Ok(t) => t,
            Err(_) => return false,
        };
        turret.cool_down.tick(time.delta());
        if turret.cool_down.finished() && input.pressed(MouseButton::Left) && **ammo > 0 {
            **ammo -= 1;
            turret.cool_down = Timer::new(Duration::from_secs_f32(0.2), false);
            true
        } else {
            false
        }
    }

    fn spawn_bullet(
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
                .insert(DespawnTimer::new(Duration::from_secs(5)))
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
            .insert(Turret::default())
            .insert(Name::from("Turret"))
            .insert(Ammo::default());
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
