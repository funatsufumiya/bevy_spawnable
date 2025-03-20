use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_spawnable::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

pub struct Hello {
    pub text: String
}

impl Spawnable for Hello {
    fn spawn<'a>(&mut self, builder: &'a mut impl GenericBuilder) -> EntityCommands<'a> {
        let mut e = builder.spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        });

        e.with_children(|root| {
            bevy::prelude::ChildBuild::spawn(root, (
                Text::new(self.text.clone()),
                TextFont {
                    font_size: 50.0,
                    ..default()
                },
                TextColor (
                    Color::WHITE.into()
                ),
                TextLayout::new_with_justify(JustifyText::Center)
            ));
        });

        e
    }}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawns(&mut Hello {
        text: "Hello, Bevy!".to_string()
    });
}