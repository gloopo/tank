use bevy::{core_pipeline::Skybox, prelude::*};

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
            ..Default::default()
        });
        app.add_systems(Startup, setup);
    }
}
fn setup(mut commands: Commands, server: Res<AssetServer>) {
    //test camera
    commands.spawn((
        Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        Skybox {
            image: server.load("skyboxes/cloudy.ktx2"), //ktx file MUST be bc7
            brightness: 1000.0,
            ..Default::default()
        }
    ));
    //map
    commands.spawn(SceneRoot(server.load("maps/world.map#Scene")));
}
