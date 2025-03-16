use anyhow::{Context, Result};
use pacm_rusty_jsc::{JSContext, JSObject, JSValue};
use pacm_rusty_jsc_macros::callback;
use std::fs;
use std::path::PathBuf;

pub fn run(input: PathBuf) -> Result<()> {
    let script = fs::read_to_string(&input)
        .with_context(|| format!("Failed to load module `{}`", input.display()))?;
    let mut context = JSContext::new();
    setup_prelude(&context);
    let result = context.evaluate_script(&script, 1);
    if let Err(ex) = result {
        anyhow::bail!("Uncaught {}", ex.to_js_string(&context).unwrap());
    }
    Ok(())
}

fn setup_prelude(context: &JSContext) {
    let require_fn = JSValue::callback(&context, Some(require_callback));
    // require()
    let mut global = context.get_global_object();
    global.set_property(&context, "require".to_string(), require_fn).unwrap();
    // foo()
    let callback = JSValue::callback(&context, Some(foo_callback));
    global.set_property(&context, "foo".to_string(), callback).unwrap();
}

#[callback]
fn require_callback(_context: JSContext, _function: JSObject, _this: JSObject, _args: &[JSValue]) -> Result<JSValue, JSValue> {
    println!("warning: `require` is not implemented.");
    Ok(JSValue::undefined(&_context))
}

#[callback]
fn foo_callback(_context: JSContext, _function: JSObject, _this: JSObject, _args: &[JSValue]) -> Result<JSValue, JSValue> {
    println!("hello from Rust land!");
    Ok(JSValue::undefined(&_context))
}
