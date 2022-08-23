use bevy::{prelude::*, utils::HashMap};
use iyes_loopless::prelude::AppLooplessStateExt;
use noise::{Fbm, NoiseFn};

use crate::{despawn, GameState};

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) width: u32,
    pub(crate) height: u32,
    map: HashMap<Position, Entity>,
}

#[derive(Debug, Copy, Clone, Component)]
pub enum Cell {
    Crop { level: u8 },
    Harvested,
}

impl Cell {
    fn from_noise_value(value: f64) -> Self {
        println!("{value}");
        let normalized = (value + 1.0) / 2.0;
        let level = if normalized < 0.25 {
            1
        } else if (0.25..0.5).contains(&normalized) {
            2
        } else if (0.5..0.75).contains(&normalized) {
            3
        } else {
            4
        };
        Self::Crop { level }
    }
}

#[derive(Component)]
struct CellGroup;

impl Field {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            map: HashMap::new(),
        }
    }

    pub fn center(&self) -> Position {
        Position(IVec2::new(
            (self.width / 2) as i32,
            (self.height / 2) as i32,
        ))
    }

    pub fn get(&self, position: Position) -> Option<Entity> {
        self.map.get(&position).copied()
    }

    pub fn get_at(&self, world_coord: Vec2) -> Option<Entity> {
        self.get(Position(world_coord.round().as_ivec2()))
    }
}

#[derive(Debug, Default, Component, Clone, Copy, Deref, DerefMut, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct Position(pub(crate) IVec2);

#[derive(Debug, Default)]
struct AssetTable {
    crop: Handle<TextureAtlas>,
    harvested: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        let mut noise = Fbm::new();
        noise.octaves = 3;
        noise.frequency = 0.5;

        println!("{noise:?}");

        app.insert_resource(Field::new(31, 15))
            .init_resource::<AssetTable>()
            .insert_resource(noise)
            .add_startup_system(Self::load_assets)
            .add_enter_system(GameState::Ready, despawn::despawn::<Cell>)
            .add_enter_system(GameState::Ready, despawn::despawn::<CellGroup>)
            .add_enter_system(GameState::Ready, Self::spawn)
            .add_system_to_stage(CoreStage::PostUpdate, Self::update_sprite);

        #[cfg(feature = "inspector")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<Position>();
        }
    }
}

impl Plugin {
    fn update_sprite(
        assets: Res<AssetTable>,
        mut cells: Query<
            (&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &Cell),
            Changed<Cell>,
        >,
    ) {
        for (mut handle, mut texture, &cell) in &mut cells {
            match cell {
                Cell::Crop { level } => {
                    *handle = assets.crop.clone();
                    texture.index = (level - 1) as usize;
                }
                Cell::Harvested => {
                    *handle = assets.harvested.clone();
                    texture.index = 0;
                }
            }
        }
    }

    fn spawn(
        mut commands: Commands,
        mut field: ResMut<Field>,
        asset_index: Res<AssetTable>,
        noise: Res<Fbm>,
    ) {
        commands
            .spawn_bundle(TransformBundle::default())
            .insert_bundle(VisibilityBundle::default())
            .insert(Name::from("Field"))
            .insert(CellGroup)
            .with_children(|field_commands| {
                for x in 0..field.width {
                    for y in 0..field.height {
                        let cell = Cell::from_noise_value(noise.get([x as f64, y as f64]));
                        let position = Position(UVec2::new(x, y).as_ivec2());
                        let entity = field_commands
                            .spawn_bundle(SpriteSheetBundle {
                                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                                sprite: TextureAtlasSprite {
                                    custom_size: Some(Vec2::ONE),
                                    ..Default::default()
                                },
                                texture_atlas: asset_index.crop.clone(),
                                ..Default::default()
                            })
                            .insert(position)
                            .insert(cell)
                            .insert(Name::from(format!("Cell ({x},{y})")))
                            .id();

                        field.map.insert(position, entity);
                    }
                }
            });
    }

    fn load_assets(
        mut index: ResMut<AssetTable>,
        asset_server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
    ) {
        index.crop = textures.add(TextureAtlas::from_grid(
            asset_server.load("crop.png"),
            Vec2::splat(32.0),
            2,
            2,
        ));
        index.harvested = textures.add(TextureAtlas::from_grid(
            asset_server.load("empty_cell.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
    }
}
