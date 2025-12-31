use bevy::{DefaultPlugins, app::App};
use bevy_trenchbroom::{TrenchBroomPlugins, config::TrenchBroomConfig};

mod game;

fn main() {
    let mut app = App::new();
    //app plugins
    app.add_plugins((
        DefaultPlugins,
        TrenchBroomPlugins(TrenchBroomConfig::new("tank"))
    ));
    //game plugins
    app.add_plugins(game::GamePlugin);
    app.run();
}
