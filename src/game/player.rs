use avian3d::prelude::{AngularDamping, AngularVelocity, Collider, Forces, LinearDamping, LockedAxes, RigidBody, RigidBodyForces, ShapeCaster, ShapeHits};
use bevy::{core_pipeline::Skybox, prelude::*};

use crate::game::environment::LoadedMap;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(FixedUpdate, (
            movement,
            rotation,
            set_is_grounded,
            set_damping.after(set_is_grounded)
        ));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Player {
    pub(crate) speed: f32,
    pub(crate) is_grounded: bool
}
impl Default for Player {
    fn default() -> Self {
        Player { speed: 10.0, is_grounded: false }
    }
}

fn spawn_player(mut commands: Commands, server: Res<AssetServer>, map: Res<LoadedMap>) {
    commands.spawn((
        Name::new("player"),
        Transform::from_xyz(0.0, 5.0, 0.0),
        Player::default(),
        SceneRoot(server.load("models/tank.glb#Scene0")),
        RigidBody::Dynamic,
        Collider::cylinder(0.5, 0.5),
        AngularDamping(0.0),
        LinearDamping(0.0),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_z(),
        ShapeCaster::new(
            Collider::cylinder(0.5, 0.1),
            Vec3::new(0.0, -0.3, 0.0),
            Quat::default(),
            Dir3::NEG_Y
            )
            .with_ignore_self(true)
            .with_max_distance(0.01),
        children![
            ((
                Transform::from_xyz(0.0, 0.0, -0.1),
                Camera3d::default(),
                Skybox {
                    image: map.skybox.clone(),
                    brightness: 1000.0,
                    ..Default::default()
                }
            ))
        ]
    ));
}

fn set_is_grounded(
    mut player_query: Query<(&ShapeHits, &mut Player)>
) {
    for (hits, mut player) in player_query.iter_mut() {
        player.is_grounded = !hits.is_empty();
    }
}

fn set_damping(
    mut player_query: Query<(&Player, &mut LinearDamping, &mut AngularDamping)>
) {
    for (player, mut linear_damp, mut angular_damp) in player_query.iter_mut() {
        if player.is_grounded {
            linear_damp.0 = 5.0;
            angular_damp.0 = 1.0;
        } else {
            linear_damp.0 = 0.0;
            angular_damp.0 = 0.0;
        }
    }
}

fn movement(
    mut player_query: Query<(Forces, &Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut forces, transform, player) in &mut player_query {
        let mut dir = 0.0;
        if input.pressed(KeyCode::KeyW) {
            dir += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            dir -= 1.0;
        }
        if player.is_grounded {
            forces.apply_linear_impulse(transform.forward() * dir * player.speed  * time.delta_secs());
        }
    }
}

fn rotation(
    mut player_query: Query<(&mut AngularVelocity, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut ang, player) in &mut player_query {
        let mut dir = 0.0;
        if input.pressed(KeyCode::KeyD) {
            dir -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            dir += 1.0;
        }
        if player.is_grounded {
            ang.0.y = dir * 60.0 * time.delta_secs();
        }
    }
}
