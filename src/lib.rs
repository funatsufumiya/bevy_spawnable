use bevy::{ecs::system::EntityCommands, prelude::*};

pub trait Spawnable {
    fn spawn<'a>(&self, builder: &'a mut impl GenericBuilder) -> EntityCommands<'a>;
}

pub trait SpawnableExt {
    fn spawns(&mut self, spawnable: &impl Spawnable) -> EntityCommands;
}

impl SpawnableExt for Commands<'_, '_> {
    fn spawns(&mut self, spawnable: &impl Spawnable) -> EntityCommands {
        spawnable.spawn(self)
    }
}

impl SpawnableExt for ChildBuilder<'_> {
    fn spawns(&mut self, spawnable: &impl Spawnable) -> EntityCommands {
        spawnable.spawn(self)
    }
}

pub trait GenericBuilder {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands;
}

impl GenericBuilder for Commands<'_, '_> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl GenericBuilder for ChildBuilder<'_> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands {
        self.spawn(bundle)
    }
}