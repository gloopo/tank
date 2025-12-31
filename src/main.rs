use avian3d::PhysicsPlugins;
use bevy::{DefaultPlugins, app::App};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_trenchbroom::{TrenchBroomPlugins, config::TrenchBroomConfig, prelude::{SceneHooks, TrenchBroomPhysicsPlugin}};
use bevy_trenchbroom_avian::AvianPhysicsBackend;

mod game;

fn main() {
    let mut app = App::new();
    //app plugins
    app.add_plugins((
        DefaultPlugins,
        EguiPlugin::default(),
        WorldInspectorPlugin::default(),
        TrenchBroomPhysicsPlugin::new(AvianPhysicsBackend),
        TrenchBroomPlugins(TrenchBroomConfig::new("tank").default_solid_scene_hooks(|| SceneHooks::new().convex_collider())),
        PhysicsPlugins::default()
    ));
    //game plugins
    app.add_plugins(game::GamePlugin);
    app.run();
}
