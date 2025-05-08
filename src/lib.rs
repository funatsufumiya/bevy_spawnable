use bevy::{ecs::{relationship::{RelatedSpawnerCommands, Relationship}, system::EntityCommands}, prelude::*};

pub trait Spawnable {
    fn spawn<'a>(&mut self, builder: &'a mut impl GenericBuilder) -> EntityCommands<'a>;
}

pub trait SpawnableExt {
    fn spawns(&mut self, spawnable: &mut impl Spawnable) -> EntityCommands;
}

impl SpawnableExt for Commands<'_, '_> {
    fn spawns(&mut self, spawnable: &mut impl Spawnable) -> EntityCommands {
        spawnable.spawn(self)
    }
}

impl<R: Relationship> SpawnableExt for RelatedSpawnerCommands<'_, R> {
    fn spawns(&mut self, spawnable: &mut impl Spawnable) -> EntityCommands {
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

impl<R: Relationship> GenericBuilder for RelatedSpawnerCommands<'_, R> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands {
        self.spawn(bundle)
    }
}