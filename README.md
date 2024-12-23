# bevy_spawnable

[![Crates.io](https://img.shields.io/crates/v/bevy_spawnable)](https://crates.io/crates/bevy_spawnable)
[![Docs.rs](https://docs.rs/bevy_spawnable/badge.svg)](https://docs.rs/bevy_spawnable)
[![License](https://img.shields.io/crates/l/bevy_spawnable)](LICENSE)

A tiny spawn utility for bevy. This crate will helps you:

- encapsulate the spawn logic into a single function or object.
- no worry about builder is `Commands` or `ChildBuilder`.

You can use this crate as a kind of like Unity's prefab as a code object, or builder to create entities which have complex structure.

( If you need more detailed spawn utility, consider [moonshine_spawn](https://github.com/Zeenobit/moonshine_spawn) as an alternative. )

## Usage

see [examples](examples)

### simple.rs

```rust
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
```

## Versions

| bevy | bevy_spawnable      |
|------|---------------------|
| 0.13 | 0.1                 |
