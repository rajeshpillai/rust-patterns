// Closure as Callback — the Fn family in practice.
//
// The big idea: accept the loosest bound that your call site
// tolerates. If you only call the closure once, ask for FnOnce. If
// you call it many times and don't mutate shared state, ask for Fn.
// Ask for FnMut only when the closure genuinely must mutate.
//
// Three API shapes covered here:
//   1. `impl Fn(...) -> ...` in an argument — static, inlined.
//   2. `Box<dyn Fn(...) -> ...>` in a struct — dynamic, stored.
//   3. A `Callback` wrapper type that owns one callback plus a
//      SubscriptionId so callers can unregister.

// ---- 1. impl Fn ---------------------------------------------------------

// Takes a Fn that we call twice. FnMut would be fine too; asking for
// Fn communicates "I won't hand this to multiple threads, but I also
// won't require exclusive access each call."
pub fn retry<T, F: Fn() -> Result<T, &'static str>>(
    attempts: u32,
    f: F,
) -> Result<T, &'static str> {
    let mut last = Err("never ran");
    for _ in 0..attempts {
        last = f();
        if last.is_ok() { return last; }
    }
    last
}

// Takes an FnOnce because it consumes the closure immediately. If
// you tried to call this with a closure that the caller also wants
// to use, they couldn't — FnOnce says "I own you, for one call."
pub fn with_cleanup<T, F, R>(value: T, f: F) -> R
where
    F: FnOnce(T) -> R,
{
    let r = f(value);
    // maybe run cleanup here
    r
}

// Takes an FnMut because we need to call it repeatedly with a mutable
// capture. Classic "iterate and accumulate" shape.
pub fn for_each<F: FnMut(i32)>(items: &[i32], mut f: F) {
    for item in items {
        f(*item);
    }
}

// ---- 2. Box<dyn Fn> in a struct (event bus) ----------------------------

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubscriptionId(u64);

pub struct EventBus<E> {
    next: u64,
    // Bound is `Fn(&E) + 'static`. `Send + Sync` only if we actually
    // share across threads — `Mutex::lock()` is NOT free, so don't
    // add it preemptively.
    callbacks: HashMap<SubscriptionId, Box<dyn Fn(&E)>>,
}

impl<E> EventBus<E> {
    pub fn new() -> Self { Self { next: 0, callbacks: HashMap::new() } }

    #[must_use = "drop the SubscriptionId and you cannot unsubscribe"]
    pub fn on<F: Fn(&E) + 'static>(&mut self, f: F) -> SubscriptionId {
        self.next += 1;
        let id = SubscriptionId(self.next);
        self.callbacks.insert(id, Box::new(f));
        id
    }

    pub fn off(&mut self, id: SubscriptionId) -> bool {
        self.callbacks.remove(&id).is_some()
    }

    pub fn emit(&self, event: &E) {
        for cb in self.callbacks.values() {
            cb(event);
        }
    }
}
impl<E> Default for EventBus<E> { fn default() -> Self { Self::new() } }

// ---- 3. RAII subscription handle ---------------------------------------

// Returning the SubscriptionId is fine, but callers keep forgetting
// to unsubscribe. Wrap the id in a RAII guard so dropping it
// auto-unsubscribes. Now you literally cannot leak a listener.
pub struct Subscription<'a, E> {
    bus: &'a mut EventBus<E>,
    id: Option<SubscriptionId>,
}

impl<E> Drop for Subscription<'_, E> {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            self.bus.off(id);
        }
    }
}

impl<'a, E> EventBus<E> {
    pub fn subscribe<F: Fn(&E) + 'static>(&'a mut self, f: F) -> Subscription<'a, E> {
        let id = self.on(f);
        Subscription { bus: self, id: Some(id) }
    }
}

fn main() {
    // 1. impl Fn
    let mut calls = 0;
    // Captures `calls` by mutable reference — this is FnMut, not Fn.
    // `retry` wants Fn, so we'd have a compile error if we passed it.
    // Use a closure with no capture instead for retry.
    for_each(&[1, 2, 3, 4, 5], |x| calls += x);
    println!("sum = {calls}");

    let r = retry(3, || Ok::<_, &'static str>("ok"));
    println!("{r:?}");

    let out = with_cleanup(vec![1, 2, 3], |mut v| {
        v.push(4);
        v.len()
    });
    println!("len = {out}");

    // 2. EventBus
    let mut bus = EventBus::<&'static str>::new();
    let _log = bus.on(|e| println!("[log] {e}"));
    bus.emit(&"user.signup");

    // 3. RAII subscription
    {
        let _sub = bus.subscribe(|e| println!("[scoped] {e}"));
        bus.emit(&"user.signup");
    } // _sub drops here; scoped listener removed automatically
    bus.emit(&"user.signup"); // only the [log] listener fires
}
