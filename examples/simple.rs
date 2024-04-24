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
        let mut e = builder.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        });

        e.with_children(|root| {
            root.spawn(
                TextBundle::from_section(
                    self.text.clone(),
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    })
                    .with_text_justify(JustifyText::Center)
                    .with_style(Style {
                        ..default()
                    })
            );
        });

        e
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawns(&mut Hello {
        text: "Hello, Bevy!".to_string()
    });
}