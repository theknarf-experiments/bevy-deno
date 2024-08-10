use bevy::prelude::*;
use bevy_deno::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(JSRuntimePlugin)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(JSModule {
                name: "test.js".to_string(),
                code: "console.log('hoho');".to_string(),
            });
            commands.spawn(JSModule {
                name: "test2.js".to_string(),
                code: "await (async () => console.log('hihi'))()".to_string(),
            });
        })
        .run();
}
