use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn::despawn, GameState};

#[derive(Debug, Clone, Copy, Default, Component)]
struct GameOverScreen;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::GameOver, Self::spawn)
            .add_exit_system(GameState::GameOver, despawn::<GameOverScreen>)
            .add_system(Self::restart.run_in_state(GameState::GameOver));
    }
}

impl Plugin {
    fn restart(mut commands: Commands, inputs: Res<Input<KeyCode>>) {
        if inputs.just_pressed(KeyCode::Space) {
            commands.insert_resource(NextState(GameState::Playing));
        }
    }

    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/Kenney-Blocks.ttf");
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .insert(GameOverScreen)
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(60.0), Val::Percent(60.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        color: Color::rgba(0.2, 0.3, 0.2, 0.99).into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(
                            TextBundle::from_section(
                                "Game Over!",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 100.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(20.0)),
                                ..default()
                            }),
                        );
                        parent.spawn_bundle(
                            TextBundle::from_section(
                                "Press <space> to restart",
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
            });
    }
}
