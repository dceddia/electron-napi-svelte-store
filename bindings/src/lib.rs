use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
  CallContext, JsUndefined,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct Ticker {
  value: u32,
  subscribers: Rc<RefCell<HashMap<u64, ThreadsafeFunction<u32, ErrorStrategy::Fatal>>>>,
  next_subscriber: u64,
}

#[napi]
impl Ticker {
  #[napi(constructor)]
  pub fn new(value: Option<u32>) -> Ticker {
    Ticker {
      value: value.unwrap_or(0),
      subscribers: Rc::new(RefCell::new(HashMap::new())),
      next_subscriber: 0,
    }
  }

  #[napi]
  pub fn increment(&mut self) -> Result<()> {
    self.value += 1;
    self.notify_subscribers()
  }

  fn notify_subscribers(&mut self) -> Result<()> {
    for (_, cbref) in self.subscribers.borrow().iter() {
      cbref.call(self.value, ThreadsafeFunctionCallMode::Blocking);
    }
    Ok(())
  }

  #[napi]
  pub fn set(&mut self, value: u32) -> Result<()> {
    self.value = value;
    self.notify_subscribers()
  }

  /// Implement Svelte's Store Contract, defined as:
  ///
  /// store = {
  ///   subscribe: (
  ///     subscription: (value: any) => void
  ///   ) => (() => void), set?: (value: any) => void
  /// }
  ///
  /// - A store must contain a .subscribe method, which must accept as its argument a subscription function.
  ///   This subscription function must be immediately and synchronously called with the store's current value
  ///   upon calling .subscribe. All of a store's active subscription functions must later be synchronously
  ///   called whenever the store's value changes.
  /// - The .subscribe method MUST return an `unsubscribe` function.
  ///   Calling an `unsubscribe` function must stop its subscription, and its corresponding subscription
  ///   function must not be called again by the store.
  /// - A store may optionally contain a `.set` method, which must accept as its argument a new value
  ///   for the store, and which synchronously calls all of the store's active subscription functions.
  ///   Such a store is called a writable store.
  #[napi]
  pub fn subscribe(&mut self, env: Env, callback: JsFunction) -> Result<JsFunction> {
    // Create a threadsafe wrapper
    //
    // The `Fatal` ErrorStrategy is best here because the argument we pass will come out as the first
    // one in JS. The `CalleeHandled` strategy, on the other hand, uses Node's calling convention,
    // where the first argument is the error (or null), and that doesn't follow the Svelte store contract.
    let tsfn: ThreadsafeFunction<u32, ErrorStrategy::Fatal> = callback
      .create_threadsafe_function(0, |ctx| ctx.env.create_uint32(ctx.value).map(|v| vec![v]))?;

    // Call once with the initial value
    tsfn.call(self.value, ThreadsafeFunctionCallMode::Blocking);

    // Save the callback so that we can call it later
    let key = self.next_subscriber;
    self.next_subscriber += 1;
    self.subscribers.borrow_mut().insert(key, tsfn);

    // Pass back an unsubscribe callback that will remove the subscription when called
    let subscribers = self.subscribers.clone();
    let unsubscribe = move |ctx: CallContext| -> Result<JsUndefined> {
      subscribers.borrow_mut().remove(&key);
      ctx.env.get_undefined()
    };

    env.create_function_from_closure("unsubscribe", unsubscribe)
  }
}
