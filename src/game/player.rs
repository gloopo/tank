use avian3d::prelude::{AngularDamping, Collider, RigidBody};
use bevy::prelude::*;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub(crate) struct Player;


fn spawn_player(mut commands: Commands, server: Res<AssetServer>) {
    info!("spawned player");
    commands.spawn((
        Name::new("player"),
        Transform::from_xyz(0.0, 10.0, 0.0),
        Player,
        SceneRoot(server.load("models/tank.glb#Scene0")),
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 1.0),
        AngularDamping(2.0)
    ));
}
