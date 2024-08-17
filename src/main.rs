// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::resolve_path;
use deno_runtime::deno_core::serde_json;
use deno_runtime::deno_core::serde_json::Value;
use deno_runtime::deno_core::serde_v8;
use deno_runtime::deno_core::ModuleCodeString;
use deno_runtime::deno_napi::v8;
use deno_runtime::deno_permissions::Permissions;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use std::path::Path;
const CURRENT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");
#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let options = WorkerOptions::default();

    let current_dir = Path::new(CURRENT_DIR);
    let main_module = resolve_path("examples/hello_runtime.js", current_dir)?;
    let permissions = Permissions::allow_all();
    let perm = PermissionsContainer::new(permissions);
    let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), perm, options);
    worker.execute_main_module(&main_module).await?;
    let code = "rustCallback('lol')";
    let result = worker.execute_script("", ModuleCodeString::from_static(code));
    match result {
        Err(err) => {
            println!("execute_mod err {:?}", err);
        }
        Ok(res) => {
            let mut scope = worker.js_runtime.handle_scope();
            let local = v8::Local::new(&mut scope, res);
            let deserialized_value = serde_v8::from_v8::<serde_json::Value>(&mut scope, local);
            match deserialized_value {
                Ok(x) => match x {
                    Value::Number(b) => {
                        println!("res {b}")
                    }
                    _ => panic!("unknown data return type"),
                },
                Err(err) => {
                    eprintln!("{err}");
                }
            }
        }
    }
    println!("execute_main_module end");
    worker.run_event_loop(false).await?;
    println!("run_event_loop  end");
    Ok(())
}
