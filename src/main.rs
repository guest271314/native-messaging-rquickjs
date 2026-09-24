/// rquickjs https://crates.io/crates/rquickjs
/// Native Messaging host (nm_rquickjs.js)
/// Globals defined for I/O in ECMAScript: std.in.read(), std.out.write(), std.stderr.write()
/// guest271314 9-23-2026

use rquickjs::{Context, Function, Object, Runtime, Value};
use std::error::Error;
use std::io::{self, Read, Write};

/// High-performance read helper. Links the 'js context lifetime to the returned raw
/// Value variant explicitly to pass Rust's strict compiler variance checks.
fn safe_read<'js>(ctx: rquickjs::Ctx<'js>, bytes_to_read: usize) -> rquickjs::Result<Value<'js>> {
  let mut buf = vec![0u8; bytes_to_read];
  let mut stdin = io::stdin().lock();
  let len = match stdin.read_exact(&mut buf) {
    Ok(_) => bytes_to_read,
    Err(_) => 0,
  };

  unsafe {
    let raw_ctx = ctx.as_raw().as_ptr();
    let raw_val =
      rquickjs::qjs::JS_NewArrayBufferCopy(raw_ctx, buf.as_ptr(), len as rquickjs::qjs::size_t);
    Ok(Value::from_raw(ctx, raw_val))
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let runtime = Runtime::new()?;
  let context = Context::full(&runtime)?;

  context.with(|ctx| {
    let global = ctx.globals();

    // 1. Bind std.in.read using our helper function
    let std_in_read = Function::new(ctx.clone(), safe_read)?;

    // 2. Bind std.out.write
    let std_out_write = Function::new(
      ctx.clone(),
      |ctx: rquickjs::Ctx<'_>, val: Value<'_>, offset: usize, length: usize| unsafe {
        let raw_ctx = ctx.as_raw().as_ptr();
        let raw_val = val.as_raw();
        let mut out_len: rquickjs::qjs::size_t = 0;
        let ptr = rquickjs::qjs::JS_GetArrayBuffer(raw_ctx, &mut out_len, raw_val);

        if !ptr.is_null() && offset + length <= out_len as usize {
          let target_slice = std::slice::from_raw_parts(ptr.add(offset), length);
          let mut stdout = io::stdout().lock();
          let _ = stdout.write_all(target_slice);
        }
      },
    )?;

    // 3. Bind std.out.flush
    let std_out_flush = Function::new(ctx.clone(), || {
      let _ = io::stdout().lock().flush();
    })?;

    // 4. Bind std.stderr.write
    let std_err_write = Function::new(
      ctx.clone(),
      |ctx: rquickjs::Ctx<'_>, val: Value<'_>, offset: usize, length: usize| unsafe {
        let raw_ctx = ctx.as_raw().as_ptr();
        let raw_val = val.as_raw();
        let mut out_len: rquickjs::qjs::size_t = 0;
        let ptr = rquickjs::qjs::JS_GetArrayBuffer(raw_ctx, &mut out_len, raw_val);

        if !ptr.is_null() && offset + length <= out_len as usize {
          let target_slice = std::slice::from_raw_parts(ptr.add(offset), length);
          let mut stderr = io::stderr().lock();
          let _ = stderr.write_all(target_slice);
        }
      },
    )?;

    // 5. Bind std.stderr.flush (💡 NEW: Native Error Stream Flusher)
    let std_err_flush = Function::new(ctx.clone(), || {
      let _ = io::stderr().lock().flush();
    })?;

    // 6. Bind std.exit
    let std_exit = Function::new(ctx.clone(), |code: i32| -> rquickjs::Result<()> {
      std::process::exit(code);
    })?;

    // Establish the QuickJS compliant object tree hierarchy
    let std_obj = Object::new(ctx.clone())?;
    let in_obj = Object::new(ctx.clone())?;
    let out_obj = Object::new(ctx.clone())?;
    let err_obj = Object::new(ctx.clone())?;

    in_obj.set("read", std_in_read)?;

    out_obj.set("write", std_out_write)?;
    out_obj.set("flush", std_out_flush)?;

    err_obj.set("write", std_err_write)?;
    err_obj.set("flush", std_err_flush)?;

    std_obj.set("in", in_obj)?;
    std_obj.set("out", out_obj)?;
    std_obj.set("stderr", err_obj)?;
    std_obj.set("exit", std_exit)?;

    global.set("std", std_obj)?;

    // 7. Embedded High-Performance JavaScript Orchestration Script
    let js_code = include_str!("nm_rquickjs.js");

    let _ = ctx.eval::<(), _>(js_code)?;
    Ok::<(), rquickjs::Error>(())
  })?;

  Ok(())
}
