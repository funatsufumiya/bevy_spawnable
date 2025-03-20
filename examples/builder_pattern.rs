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
    fn spawn<'a>(&mut self, builder: &'a mut impl GenericBuilder) -> EntityCommands<'a> {
        builder.spawn((
            Text::new(self.text.clone()),
            TextFont {
                font_size: self.font_size,
                ..default()
            },
            TextColor (
                self.color.into()
            ),
            TextLayout::new_with_justify(JustifyText::Center)
        ))
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
            // color: Color::RED,
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            font_size: 50.0,
        },
        text_type_c: MyTextBuilder {
            // color: Color::BLUE,
            color: Color::linear_rgb(0.0, 0.0, 1.0),
            font_size: 60.0,
        },
    };

    commands.insert_resource(builders);
}

fn setup(
    mut commands: Commands,
    builders: Res<TextBuilders>,
) {
    commands.spawn(Camera2d::default());

    commands.spawn(Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }).with_children(|r| {
        r.spawns(&mut builders.text_type_a.text("Text A".to_string()));
        r.spawns(&mut builders.text_type_b.text("Text B".to_string()));
        r.spawns(&mut builders.text_type_c.text("Text C".to_string()));
    });
}