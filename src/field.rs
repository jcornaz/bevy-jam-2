use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) width: usize,
    pub(crate) height: usize,
    #[allow(unused)]
    cells: Vec<Cell>,
}

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Crop,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::new();
        cells.resize(width * height, Cell::Crop);
        Self {
            width,
            height,
            cells,
        }
    }
}

#[derive(Debug, Default, Component)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
#[allow(unused)]
struct FieldPos {
    #[cfg_attr(feature = "inspector", inspectable(read_only))]
    x: usize,
    #[cfg_attr(feature = "inspector", inspectable(read_only))]
    y: usize,
}

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Field::new(30, 15))
            .add_startup_system(Self::spawn_cell_entities);

        #[cfg(feature = "inspector")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<FieldPos>();
        }
    }
}

impl Plugin {
    pub fn spawn_cell_entities(
        mut commands: Commands,
        field: Res<Field>,
        asset_server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
    ) {
        let crop_texture = textures.add(TextureAtlas::from_grid(
            asset_server.load("crop.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
        for x in 0..field.width {
            for y in 0..field.height {
                let mut entity = commands.spawn_bundle(SpriteSheetBundle {
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    texture_atlas: crop_texture.clone(),
                    ..Default::default()
                });
                entity.insert(FieldPos { x, y });

                #[cfg(feature = "inspector")]
                entity.insert(Name::from(format!("Cell ({x},{y})")));
            }
        }
    }
}
