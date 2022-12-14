use bevy::prelude::*;

mod game_over;
mod ready;

#[derive(Default)]
pub struct Plugins;

impl PluginGroup for Plugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(game_over::Plugin::default())
            .add(ready::Plugin::default());
    }
}

fn spawn_screen<C: Component + Default>(
    commands: &mut Commands,
    children: impl FnOnce(&mut ChildBuilder),
) {
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
        .insert(C::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(50.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::hex("8399b4").unwrap().into(),
                    ..Default::default()
                })
                .with_children(children);
        });
}
