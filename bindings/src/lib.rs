use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread::{self, JoinHandle},
  time::Duration,
};

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction},
  JsNumber,
};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct JsRepeater {
  handle1: Option<JoinHandle<Result<()>>>,
  handle2: Option<JoinHandle<Result<()>>>,
  quit: Arc<AtomicBool>,
}

#[napi]
impl JsRepeater {
  #[napi(constructor)]
  pub fn new(callback1: JsFunction, callback2: JsFunction) -> Result<Self> {
    // Create a threadsafe function around each callback
    //
    // Other things to try:
    //   - call create_threadsafe_function twice on the same callback (has same effect)
    //   - only create 1 threadsafe function (doesn't deadlock-panic, but does panic when failing to release)
    let cb1: ThreadsafeFunction<u32> = callback1.create_threadsafe_function(0, send_update)?;
    let cb2: ThreadsafeFunction<u32> = callback2.create_threadsafe_function(0, send_update)?;

    let quit = Arc::new(AtomicBool::new(false));
    let handle1 = spawn_worker("ONE".to_string(), quit.clone(), cb1);
    let handle2 = spawn_worker("TWO".to_string(), quit.clone(), cb2);

    Ok(JsRepeater {
      handle1: Some(handle1),
      handle2: Some(handle2),
      quit,
    })
  }
}

impl Drop for JsRepeater {
  fn drop(&mut self) {
    self.quit.store(true, Ordering::SeqCst);

    if let Some(thread) = self.handle1.take() {
      println!("joining handle1");
      let _ = thread.join();
      println!("joined handle1");
    }

    if let Some(thread) = self.handle2.take() {
      println!("joining handle2");
      let _ = thread.join();
      println!("joined handle2");
    }
  }
}

fn send_update(ctx: ThreadSafeCallContext<u32>) -> Result<Vec<JsNumber>> {
  ctx.env.create_uint32(ctx.value).map(|v| vec![v])
}

fn spawn_worker(
  name: String,
  should_quit: Arc<AtomicBool>,
  ts_callback: ThreadsafeFunction<u32>,
) -> JoinHandle<Result<()>> {
  thread::spawn(move || {
    let mut i = 0;

    while !should_quit.load(Ordering::SeqCst) {
      thread::sleep(Duration::from_millis(3));
      let status = ts_callback.call(
        Ok(i),
        napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
      );
      //println!("called function {} with {}, {}", name, i, status);
      i += 1;
    }

    /*
    println!(
      "done with loop, callback is aborted? {:?}",
      ts_callback.aborted()
    );
    */

    Ok(())
  })
}
