use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) width: u32,
    pub(crate) height: u32,
    map: HashMap<Position, Entity>,
}

#[derive(Debug, Copy, Clone, Component)]
pub enum Cell {
    Crop,
    Harvested,
}

impl Cell {
    pub fn harvest(&mut self) {
        *self = Cell::Harvested;
    }
}

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
        app.insert_resource(Field::new(31, 15))
            .init_resource::<AssetTable>()
            .add_startup_system_to_stage(StartupStage::PreStartup, Self::load_assets)
            .add_startup_system(Self::spawn)
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
        mut cells: Query<(&mut Handle<TextureAtlas>, &Cell), Changed<Cell>>,
    ) {
        for (mut handle, &cell) in &mut cells {
            *handle = match cell {
                Cell::Crop => assets.crop.clone(),
                Cell::Harvested => assets.harvested.clone(),
            }
        }
    }

    fn spawn(mut commands: Commands, mut field: ResMut<Field>, asset_index: Res<AssetTable>) {
        for x in 0..field.width {
            for y in 0..field.height {
                let position = Position(UVec2::new(x, y).as_ivec2());
                let mut entity = commands.spawn_bundle(SpriteSheetBundle {
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    texture_atlas: asset_index.crop.clone(),
                    ..Default::default()
                });
                entity.insert(position).insert(Cell::Crop);

                #[cfg(feature = "inspector")]
                entity.insert(Name::from(format!("Cell ({x},{y})")));

                field.map.insert(position, entity.id());
            }
        }
    }

    fn load_assets(
        mut index: ResMut<AssetTable>,
        asset_server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
    ) {
        index.crop = textures.add(TextureAtlas::from_grid(
            asset_server.load("crop.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
        index.harvested = textures.add(TextureAtlas::from_grid(
            asset_server.load("harvested.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
    }
}
