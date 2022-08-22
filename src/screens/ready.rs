use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn::despawn, GameState};

use super::spawn_screen;

#[derive(Debug, Clone, Copy, Default, Component)]
struct ReadyScreen;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Ready, Self::spawn)
            .add_exit_system(GameState::Ready, despawn::<ReadyScreen>)
            .add_system(Self::start.run_in_state(GameState::Ready));
    }
}

impl Plugin {
    fn start(mut commands: Commands, input: Res<Input<KeyCode>>) {
        if input.just_pressed(KeyCode::Space) {
            commands.insert_resource(NextState(GameState::Playing));
        }
    }

    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/Kenney-Blocks.ttf");
        spawn_screen::<ReadyScreen>(&mut commands, |parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Press <space> to start",
                    TextStyle {
                        font,
                        color: Color::BLACK,
                        font_size: 50.0,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                }),
            );
        });
    }
}
