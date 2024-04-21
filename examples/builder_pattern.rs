use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_spawnable::*;

// This example shows how to give assets to the builders generates spawnables

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreStartup, setup_builders)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
pub struct TextBuilders {
    pub text_type_a : MyTextBuilder,
    pub text_type_b : MyTextBuilder,
    pub text_type_c : MyTextBuilder,
}

pub struct MyTextBuilder {
    pub color: Color,
    pub font_size: f32,
}

impl MyTextBuilder {
    fn text(&self, text: String) -> MyText {
        MyText {
            text: text,
            color: self.color,
            font_size: self.font_size,
        }
    }
}

pub struct MyText {
    pub text: String,
    pub color: Color,
    pub font_size: f32,
}

impl Spawnable for MyText {
    fn spawn<'a>(&self, builder: &'a mut impl GenericBuilder) -> EntityCommands<'a> {
        builder.spawn(
            TextBundle::from_section(
                self.text.clone(),
                TextStyle {
                    font_size: self.font_size,
                    color: self.color,
                    ..default()
                })
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    ..default()
                })
        )
    }
}

fn setup_builders (
    mut commands: Commands,
) {
    // you can give any assets to the builders here

    let builders = TextBuilders {
        text_type_a: MyTextBuilder {
            color: Color::WHITE,
            font_size: 40.0,
        },
        text_type_b: MyTextBuilder {
            color: Color::RED,
            font_size: 50.0,
        },
        text_type_c: MyTextBuilder {
            color: Color::BLUE,
            font_size: 60.0,
        },
    };

    commands.insert_resource(builders);
}

fn setup(
    mut commands: Commands,
    builders: Res<TextBuilders>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|r| {
        r.spawns(&builders.text_type_a.text("Text A".to_string()));
        r.spawns(&builders.text_type_b.text("Text B".to_string()));
        r.spawns(&builders.text_type_c.text("Text C".to_string()));
    });
}