use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) width: u32,
    pub(crate) height: u32,
    #[allow(unused)]
    cells: Vec<Cell>,
}

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Crop,
    Harvested,
}

#[derive(Debug, Clone, Copy, Component)]
struct CellEntity;

impl Field {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = Vec::new();
        cells.resize((width * height) as usize, Cell::Crop);
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn center(&self) -> Position {
        Position(IVec2::new(
            (self.width / 2) as i32,
            (self.height / 2) as i32,
        ))
    }

    fn get(&self, position: Position) -> Option<Cell> {
        self.index_of(position)
            .and_then(|i| self.cells.get(i))
            .copied()
    }

    pub fn harvest(&mut self, position: Position) {
        if let Some(cell) = self.index_of(position).and_then(|i| self.cells.get_mut(i)) {
            *cell = Cell::Harvested;
        }
    }

    fn index_of(&self, Position(v): Position) -> Option<usize> {
        if v.x < 0 || v.x >= self.width as i32 {
            return None;
        }
        if v.y < 0 || v.y >= self.height as i32 {
            return None;
        }
        Some((v.y as usize * self.width as usize) + v.x as usize)
    }
}

#[derive(Debug, Default, Component, Clone, Copy, Deref, DerefMut)]
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
            .add_startup_system(Self::spawn_cell_entities)
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
        field: Res<Field>,
        assets: Res<AssetTable>,
        mut cells: Query<(&mut Handle<TextureAtlas>, &Position), With<CellEntity>>,
    ) {
        if !field.is_changed() {
            return;
        }

        for (mut cell, pos) in &mut cells {
            *cell = match field.get(*pos) {
                Some(Cell::Crop) => assets.crop.clone(),
                Some(Cell::Harvested) => assets.harvested.clone(),
                None => continue,
            }
        }
    }

    fn spawn_cell_entities(
        mut commands: Commands,
        field: Res<Field>,
        asset_index: Res<AssetTable>,
    ) {
        for x in 0..field.width {
            for y in 0..field.height {
                let mut entity = commands.spawn_bundle(SpriteSheetBundle {
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    texture_atlas: asset_index.crop.clone(),
                    ..Default::default()
                });
                entity
                    .insert(Position(UVec2::new(x, y).as_ivec2()))
                    .insert(CellEntity);

                #[cfg(feature = "inspector")]
                entity.insert(Name::from(format!("Cell ({x},{y})")));
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
