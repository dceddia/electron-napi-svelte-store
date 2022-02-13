use napi::{bindgen_prelude::*, CallContext, JsUndefined, Ref};
use std::collections::HashMap;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

type JsFunctionRef = Ref<()>;

#[napi]
pub struct Ticker {
  value: u32,
  subscribers: HashMap<u64, JsFunctionRef>,
  next_subscriber: u64,
}

#[napi]
impl Ticker {
  #[napi(constructor)]
  pub fn new(value: Option<u32>) -> Ticker {
    Ticker {
      value: value.unwrap_or(0),
      subscribers: HashMap::new(),
      next_subscriber: 0,
    }
  }

  #[napi]
  pub fn increment(&mut self, env: Env) -> Result<()> {
    self.value += 1;
    println!("increment! now {}", self.value);
    self.notify_subscribers(env)
  }

  fn notify_subscribers(&mut self, env: Env) -> Result<()> {
    for (_, cbref) in &self.subscribers {
      // Get the callback out of the Ref that was saved earlier in `subscribe`
      let callback: JsFunction = env.get_reference_value(cbref)?;

      // Call it
      let args = vec![env.create_uint32(self.value)?];
      callback.call(None, &args)?;
    }
    Ok(())
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
    // Call once with the initial value
    callback.call(None, vec![env.create_uint32(self.value)?].as_slice())?;

    // Explicitly create a reference to the callback so that it doesn't get
    // garbage collected after returning.
    let cbref = env.create_reference(callback)?;

    // Save the callback so that we can call it later
    let key = self.next_subscriber;
    self.subscribers.insert(key, cbref);
    self.next_subscriber += 1;

    let unsubscribe = |ctx: CallContext| -> Result<JsUndefined> {
      // self.subscribers.remove(&key);
      println!("should unsubscribe!");
      ctx.env.get_undefined()
    };

    env.create_function_from_closure("unsubscribe", unsubscribe)
  }
}
