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
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        Skybox {
            image: server.load("skyboxes/ryfjallet_cubemap_bc7.ktx2"), //ktx file MUST be bc7
            brightness: 1000.0,
            ..Default::default()
        }
    ));
}
