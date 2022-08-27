use bevy::{prelude::*, render::texture::ImageSettings};
use enemy::PlayerHit;
use iyes_loopless::prelude::*;

mod barrier;
mod camera;
mod combine;
mod despawn;
mod enemy;
mod field;
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

fn main() {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::hex("5a655a").unwrap()))
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_loopless_state(GameState::Ready)
        .add_plugin(camera::Plugin::default())
        .add_plugin(mouse::Plugin::default())
        .add_plugin(field::Plugin::default())
        .add_plugin(combine::Plugin::default())
        .add_plugin(enemy::Plugin::default())
        .add_plugin(turret::Plugin::default())
        .add_plugin(barrier::Plugin::default())
        .add_plugins(screens::Plugins::default())
        .add_system_set(movement::systems())
        .add_system_set(despawn::systems())
        .add_system(game_over.run_in_state(GameState::Playing))
        .run();
}

fn game_over(mut commands: Commands, mut player_hits: EventReader<PlayerHit>) {
    if player_hits.iter().count() > 0 {
        commands.insert_resource(NextState(GameState::GameOver));
    }
}
