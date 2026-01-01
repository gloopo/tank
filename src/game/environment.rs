use bevy::prelude::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
            ..Default::default()
        });
        app.add_systems(PreStartup, setup);
    }
}

#[derive(Resource)]
pub(crate) struct LoadedMap {
    pub(crate) skybox: Handle<Image>,
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(SceneRoot(server.load("maps/world.map#Scene")));
    commands.insert_resource(LoadedMap {
        skybox: server.load("skyboxes/cloudy.ktx2")
    });
}
