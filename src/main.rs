use avian3d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::{DefaultPlugins, app::App, log::LogPlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_trenchbroom::{TrenchBroomPlugins, config::TrenchBroomConfig, prelude::{SceneHooks, TrenchBroomPhysicsPlugin}};
use bevy_trenchbroom_avian::AvianPhysicsBackend;

mod game;

fn main() {
    let mut app = App::new();
    //app plugins
    app.add_plugins((
        DefaultPlugins
            .set(LogPlugin {
                filter: format!(
                    concat!(
                        "{default},",
                        "calloop::loop_logic=error,",
                    ),
                    default = bevy::log::DEFAULT_FILTER
                ),
                ..Default::default()
            }),
        EguiPlugin::default(),
        WorldInspectorPlugin::default(),
        TrenchBroomPhysicsPlugin::new(AvianPhysicsBackend),
        TrenchBroomPlugins(TrenchBroomConfig::new("tank").default_solid_scene_hooks(|| SceneHooks::new().convex_collider())),
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default()
    ));
    //game plugins
    app.add_plugins(game::GamePlugin);
    app.run();
}
