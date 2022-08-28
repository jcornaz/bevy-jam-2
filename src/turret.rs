use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use itertools_num::linspace;
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    combine::{Harvested, Harvester},
    despawn::{despawn, DespawnTimer},
    enemy::Enemy,
    mouse::Cursor,
    movement::Velocity,
    GameState,
};

pub const MAX_AMMO: u32 = 20;

#[derive(Debug, Default)]
struct AssetTable {
    turret: Handle<TextureAtlas>,
    bullet: Handle<TextureAtlas>,
    item: Handle<TextureAtlas>,
}

#[derive(Debug, Clone, Copy)]
enum TurretMode {
    BASE,
    FAST,
    SHOTGUN,
    SPLIT,
    REVERSE,
    NUKE,
}

#[derive(Debug, Clone, Component)]
struct Turret {
    cool_down: Timer,
    mode: TurretMode,
}

impl Default for Turret {
    fn default() -> Self {
        Self {
            cool_down: Timer::new(Duration::ZERO, false),
            mode: TurretMode::SPLIT,
        }
    }
}

#[derive(Debug, Clone, Default, Component, Deref, DerefMut)]
pub struct Ammo(u32);

#[derive(Debug, Clone, Component, Default)]
struct Bullet;

#[derive(Debug, Clone, Component)]
pub struct Item {
    mode: TurretMode,
}

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .add_startup_system(Self::load_assets)
            .add_enter_system(GameState::Ready, despawn::<Turret>)
            .add_enter_system(GameState::Ready, despawn::<Item>)
            .add_enter_system(GameState::Ready, Self::spawn_turret)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::aim.run_in_state(GameState::Playing),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(Self::spawn_bullet.run_if(Self::shoot))
                    .with_system(Self::kill_enemy)
                    .with_system(Self::collect_item)
                    .into(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::reload.run_in_state(GameState::Playing),
            );
    }
}

impl Plugin {
    fn reload(mut harvests: EventReader<Harvested>, mut ammos: Query<&mut Ammo>) {
        const AMMO_PER_CROP_CELL: u32 = 1;
        let delta_ammo = harvests.iter().count() as u32 * AMMO_PER_CROP_CELL;
        if delta_ammo == 0 {
            return;
        }
        for mut ammo in &mut ammos {
            **ammo = (**ammo + delta_ammo).min(MAX_AMMO);
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
        turrets: Query<(&Transform, &Turret), With<Turret>>,
        assets: Res<AssetTable>,
    ) {
        for (turret_transform, turret) in &turrets {
            let mut transform = *turret_transform;
            transform.translation -= Vec3::Z * 0.5; // To be rendered behind the turret

            let velocity = match turret.mode {
                TurretMode::FAST => 20.0,
                _ => 10.0,
            };

            let shots: Vec<f32> = match turret.mode {
                TurretMode::SHOTGUN => linspace(-0.2, 0.2, 3).into_iter().collect(),
                TurretMode::SPLIT => linspace(-0.2, 0.2, 2).into_iter().collect(),
                TurretMode::NUKE => linspace(-PI, PI, 30).into_iter().collect(),
                TurretMode::REVERSE => vec![-PI],
                _ => vec![0.0],
            };

            for shot_angle in shots {
                let mut shot_transform = transform;
                shot_transform.rotate_z(shot_angle);
                shot_transform.translation += shot_transform.local_x() * 0.6;

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: assets.bullet.clone(),
                        transform: shot_transform,
                        sprite: TextureAtlasSprite {
                            custom_size: Some(Vec2::ONE),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Velocity(shot_transform.local_x().truncate() * velocity))
                    .insert(Bullet::default())
                    .insert(DespawnTimer::new(Duration::from_secs(5)))
                    .insert(Name::from("Bullet"));
            }
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
                turret_transform.translation -= combine_transform.local_x() * 0.15;
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
        bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
        enemies: Query<(Entity, &GlobalTransform, &Transform), With<Enemy>>,
        assets: Res<AssetTable>,
    ) {
        for (bullet_entity, bullet) in &bullets {
            for (enemy_entity, enemy, enemy_transform) in &enemies {
                let dist_squared = (bullet.translation().truncate()
                    - enemy.translation().truncate())
                .length_squared();
                if dist_squared < 0.3 {
                    let mut rng = thread_rng();

                    if rng.gen_bool(0.1) {
                        let turret_mode = match rng.gen_range(0..100) {
                            0..=10 => TurretMode::BASE,
                            11..=40 => TurretMode::FAST,
                            41..=70 => TurretMode::SHOTGUN,
                            71..=80 => TurretMode::SPLIT,
                            90..=100 => TurretMode::NUKE,
                            _ => TurretMode::BASE,
                        };

                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                texture_atlas: assets.item.clone(),
                                transform: *enemy_transform,
                                sprite: TextureAtlasSprite {
                                    custom_size: Some(Vec2::ONE * 0.3),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Item { mode: turret_mode })
                            .insert(DespawnTimer::new(Duration::from_secs(5)))
                            .insert(Name::from("Item"));
                    }

                    commands.entity(enemy_entity).despawn_recursive();
                    commands.entity(bullet_entity).despawn_recursive();
                }
            }
        }
    }

    fn collect_item(
        mut commands: Commands,
        combines: Query<&GlobalTransform, (With<Harvester>, Without<Item>)>,
        items: Query<(Entity, &Item, &GlobalTransform)>,
        mut turrets: Query<&mut Turret>,
    ) {
        for combine in &combines {
            for (item_entity, item, item_transform) in &items {
                let dist_squared = (combine.translation().truncate()
                    - item_transform.translation().truncate())
                .length_squared();

                if dist_squared < 0.3 {
                    for mut turret in &mut turrets {
                        turret.mode = item.mode
                    }

                    commands.entity(item_entity).despawn_recursive();
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
            server.load("sprites/turret.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
        table.bullet = textures.add(TextureAtlas::from_grid(
            server.load("sprites/bullet.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
        table.item = textures.add(TextureAtlas::from_grid(
            server.load("sprites/item.png"),
            Vec2::splat(17.0),
            1,
            1,
        ));
    }
}

// fn linspace(range: RangeInclusive<f32>, count: u8) -> Vec<f32> {
//     let mut vec = Vec::with_capacity(count as usize);
//     vec.push(*range.start());
//     let step_size = (*range.end() - *range.start()) / (count - 1) as f32;
//     for i in 1..count {
//         vec.push(i as f32 * step_size);
//     }
//     vec.push(*range.end());
//     vec
// }
