use std::fmt::Display;

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_kira_audio::prelude::*;
use combine::Harvested;
use enemy::PlayerHit;
use field::{Cell, Field};
use iyes_loopless::prelude::*;

mod barrier;
mod camera;
mod combine;
mod despawn;
mod enemy;
mod field;
mod hud;
mod mouse;
mod movement;
mod screens;
mod turret;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GameState {
    Ready,
    Playing,
    GameOver,
}

#[derive(Debug, Default)]
struct Fonts {
    main: Handle<Font>,
}

#[derive(Default, Deref, DerefMut)]
struct Score(f32);

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0}%", self.0)
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::hex("5a655a").unwrap()))
        .insert_resource(WindowDescriptor {
            width: 1800.0,
            height: 900.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin::default());

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_loopless_state(GameState::Ready)
        .init_resource::<Fonts>()
        .init_resource::<Score>()
        .add_startup_system(load_fonts)
        .add_plugin(camera::Plugin::default())
        .add_plugin(mouse::Plugin::default())
        .add_plugin(field::Plugin::default())
        .add_plugin(combine::Plugin::default())
        .add_plugin(enemy::Plugin::default())
        .add_plugin(turret::Plugin::default())
        .add_plugin(barrier::Plugin::default())
        .add_plugin(hud::Plugin::default())
        .add_plugins(screens::Plugins::default())
        .add_system_set(movement::systems())
        .add_system_set(despawn::systems())
        .add_enter_system(GameState::Ready, reset_score)
        .add_system(update_score.run_in_state(GameState::Playing))
        .add_system(game_over.run_in_state(GameState::Playing))
        .run();
}

fn game_over(mut commands: Commands, mut player_hits: EventReader<PlayerHit>) {
    if player_hits.iter().count() > 0 {
        commands.insert_resource(NextState(GameState::GameOver));
    }
}

fn load_fonts(mut fonts: ResMut<Fonts>, asset_server: Res<AssetServer>) {
    fonts.main = asset_server.load("fonts/Kenney-Blocks.ttf");
}

fn reset_score(mut score: ResMut<Score>) {
    **score = 0.0;
}

fn update_score(
    mut harvested: EventReader<Harvested>,
    mut score: ResMut<Score>,
    cells: Query<&Cell>,
    field: Res<Field>,
) {
    if harvested.iter().count() == 0 {
        return;
    }
    let count = cells
        .iter()
        .filter(|c| matches!(c, Cell::Harvested))
        .count();
    **score = 100.0 * count as f32 / (field.width * field.height) as f32;
}
