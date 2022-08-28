use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    combine::Harvested,
    despawn::despawn,
    field::{Cell, Field},
    turret::Ammo,
    Fonts, GameState,
};

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
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(Self::update_ammo)
                    .with_system(Self::update_score)
                    .into(),
            );
    }
}

impl Plugin {
    fn spawn(mut commands: Commands, fonts: Res<Fonts>, asset_server: Res<AssetServer>) {
        let color = Color::hex("8399b4").unwrap().into();
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexEnd,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .insert(Hud)
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(ImageBundle {
                            image: UiImage(asset_server.load("icons/ammo.png")),
                            style: Style {
                                size: Size::new(Val::Px(70.0), Val::Px(70.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
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
                                    margin: UiRect::new(
                                        Val::Px(0.0),
                                        Val::Px(20.0),
                                        Val::Px(20.0),
                                        Val::Px(20.0),
                                    ),
                                    ..Default::default()
                                }),
                            )
                            .insert(AmmoText);
                    });
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(
                                TextBundle::from_section(
                                    "0%",
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
                            .insert(ScoreText);
                        parent.spawn_bundle(ImageBundle {
                            image: UiImage(asset_server.load("icons/field.png")),
                            style: Style {
                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                margin: UiRect::new(
                                    Val::Px(0.0),
                                    Val::Px(20.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            });
    }

    fn update_ammo(
        ammos: Query<&Ammo, Changed<Ammo>>,
        mut texts: Query<&mut Text, With<AmmoText>>,
    ) {
        for ammo in &ammos {
            for mut text in &mut texts {
                text.sections[0].value = ammo.to_string();
            }
        }
    }

    fn update_score(
        mut harvested: EventReader<Harvested>,
        cells: Query<&Cell>,
        field: Res<Field>,
        mut texts: Query<&mut Text, With<ScoreText>>,
    ) {
        if harvested.iter().count() == 0 {
            return;
        }
        let count = cells
            .iter()
            .filter(|c| matches!(c, Cell::Harvested))
            .count();
        let percentage = 100.0 * count as f32 / (field.width * field.height) as f32;
        for mut text in &mut texts {
            text.sections[0].value = format!("{:.0}%", percentage);
        }
    }
}
