use anyhow::anyhow;
use deno_runtime::{
    deno_core::{
        error::AnyError,
        extension, op2, resolve_path,
        serde_json::{self},
        serde_v8, ModuleCodeString, OpState, *,
    },
    deno_napi::v8,
    deno_permissions::{Permissions, PermissionsContainer},
    worker::{MainWorker, WorkerOptions},
};
use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc};
#[op2(async)]
#[serde]
pub async fn op_redirect(state: Rc<RefCell<OpState>>, #[string] s: String) -> Result<(), AnyError> {
    println!("{s}");
    Ok(())
}
#[op2(fast)]
pub fn op_set_header(state: &mut OpState) {}

struct SharedData {}

type SData = Arc<SharedData>;
extension!(
    http_filter_ext,
    ops = [op_redirect],
    esm = ["examples/filter_ext.ts"],
    options = {
        shared: SData,
    },
    state = |state, options| {
        state.put(options.shared);
    },
    docs = "A small sample extension",
);
const CURRENT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");
async fn call() -> Result<(), AnyError> {
    let shared = Arc::new(SharedData {});
    let options = WorkerOptions {
        extensions: vec![http_filter_ext::init_ops_and_esm(shared)],
        ..Default::default()
    };
    let current_dir = Path::new(CURRENT_DIR).join("examples");
    let main_module = resolve_path("filter.ts", &current_dir)?;
    let permissions = Permissions::allow_all();
    let perm = PermissionsContainer::new(permissions);
    let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), perm, options);
    worker.execute_main_module(&main_module).await?;
    // Q1: how to get js http_request_header_filter hook in rust and store it for later use?
    // Q2:
    let req = BridgeHttpRequest { path: "/".into() };
    let resp = BridgeHttpResponse {};
    // how I call http_request_header_filter in rust and pass req/resp to it?
    // RUST_FUNCTION_http_request_header_filter(req, resp)
    println!("execute_main_module end");
    worker.run_event_loop(false).await?;
    println!("run_event_loop  end");
    Ok(())
}
struct BridgeHttpRequest {
    path: String,
}
impl BridgeHttpRequest {
    pub fn url(&self) -> String {
        todo!()
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn get_header(&self, k: String) -> String {
        todo!()
    }
    pub fn redirect(&self, k: String) {
        todo!()
    }
    pub fn redirect(&self) -> String {
        todo!()
    }
}

struct BridgeHttpResponse {}

impl BridgeHttpResponse {
    pub fn body(&self) -> Vec<u8> {}
    pub fn set_header(&self, k: String, v: String) {}
    pub fn get_header(&self, k: String) -> String {}
}

fn eval(context: &mut MainWorker, code: &'static str) -> anyhow::Result<serde_json::Value> {
    let res = context.execute_script("<anon>", ModuleCodeString::from_static(code));
    match res {
        Ok(global) => {
            let scope = &mut context.js_runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(anyhow!("Cannot deserialize value: {err:?}")),
            }
        }
        Err(err) => Err(anyhow!("Evaling error: {err:?}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::call;
    #[tokio::test]
    async fn test_load() {
        call().await.unwrap();
    }
}
fn main() {}
