use bevy::{prelude::*, render::texture::ImageSettings};

mod camera;
mod field;

fn main() {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_plugin(field::Plugin::default())
        .add_plugin(camera::Plugin::default())
        // .add_system(print_asset_state)
        .run();
}

// fn print_asset_state(q: Query<&Handle<TextureAtlas>>, server: Res<AssetServer>) {
//     for h in q.iter().take(1) {
//         println!("{:?}", server.get_load_state(h));
//     }
// }
