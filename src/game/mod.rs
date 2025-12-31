use bevy::prelude::*;
pub mod environment;
pub mod player;

pub(crate) struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(environment::EnvironmentPlugin);
    }
}

//game plugin will manage game state like scores, loaded map, players, etc...
