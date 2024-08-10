use anyhow::Context;
use bevy::prelude::*;
use deno_core::*;
use std::rc::Rc;

#[derive(Component, Clone, Debug)]
struct JSModule {
    name: String,
    code: String,
}

fn js_runner_system(query: Query<&JSModule>) {
    for js_module in &query {
        let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
            ..Default::default()
        });

        let main_module = resolve_path(
            js_module.name.clone().as_str(),
            &std::env::current_dir()
                .context("Unable to get current working directory")
                .unwrap(),
        )
        .unwrap();

        let future = async move {
            let mod_id = js_runtime
                .load_main_es_module_from_code(&main_module, js_module.code.clone())
                .await?;

            let result = js_runtime.mod_evaluate(mod_id);
            js_runtime.run_event_loop(Default::default()).await?;
            result.await
        };

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
            .unwrap();
    }
}

pub struct JSRuntimePlugin;

impl Plugin for JSRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, js_runner_system);
    }
}

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
