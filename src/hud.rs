use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn::despawn, turret::Ammo, Fonts, GameState};

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct AmmoText;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, Self::spawn)
            .add_exit_system(GameState::Playing, despawn::<Hud>)
            .add_system(Self::update_score.run_in_state(GameState::Playing));
    }
}

impl Plugin {
    fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                color: Color::hex("bd956a").unwrap().into(),
                ..Default::default()
            })
            .insert(Hud)
            .with_children(|parent| {
                parent
                    .spawn_bundle(
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font: fonts.main.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..Default::default()
                        }),
                    )
                    .insert(AmmoText);
            });
    }

    fn update_score(
        ammos: Query<&Ammo, Changed<Ammo>>,
        mut texts: Query<&mut Text, With<AmmoText>>,
    ) {
        for ammo in &ammos {
            for mut text in &mut texts {
                text.sections[0].value = ammo.to_string();
            }
        }
    }
}
