use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

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
    bullet_sound: Handle<AudioSource>,
}

#[derive(Debug, Clone)]
enum TurretMode {
    BASE,
    BETTER,
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
            mode: TurretMode::BASE,
        }
    }
}

#[derive(Debug, Clone, Default, Component, Deref, DerefMut)]
pub struct Ammo(u32);

#[derive(Debug, Clone, Component, Default)]
struct Bullet;

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Item;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .add_startup_system(Self::load_assets)
            .add_enter_system(GameState::Ready, despawn::<Turret>)
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
        audio: Res<Audio>,
    ) {
        audio.play(assets.bullet_sound.clone());
        for (turret_transform, turret) in &turrets {
            let mut transform = *turret_transform;
            transform.translation -= Vec3::Z * 0.5; // To be rendered behind the turret
            transform.translation += transform.local_x() * 0.6;

            let velocity = match turret.mode {
                TurretMode::BASE => 10.0,
                TurretMode::BETTER => 40.0,
            };
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: assets.bullet.clone(),
                    transform,
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity(transform.local_x().truncate() * velocity))
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
                        .insert(Item::default())
                        .insert(DespawnTimer::new(Duration::from_secs(5)))
                        .insert(Name::from("Item"));

                    commands.entity(enemy_entity).despawn_recursive();
                    commands.entity(bullet_entity).despawn_recursive();
                }
            }
        }
    }

    fn collect_item(
        mut commands: Commands,
        combines: Query<&GlobalTransform, (With<Harvester>, Without<Item>)>,
        items: Query<(Entity, &GlobalTransform), With<Item>>,
        mut turrets: Query<&mut Turret>,
    ) {
        for combine in &combines {
            for (item_entity, item_transform) in &items {
                let dist_squared = (combine.translation().truncate()
                    - item_transform.translation().truncate())
                .length_squared();

                if dist_squared < 0.3 {
                    for mut turret in &mut turrets {
                        turret.mode = TurretMode::BETTER
                    }

                    commands.entity(item_entity).despawn_recursive();

                    println!("collected!")
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
        table.bullet_sound = server.load("sounds/bullet-sound.wav");
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
