// Mediator — a central hub routes messages between peers so peers
// don't hold references to one another. In Rust the natural carrier
// is a channel. This example uses std::sync::mpsc for portability;
// in async code you'd reach for tokio::sync::broadcast.
//
// Three moves:
//   1. A ChatRoom that owns a Vec of sender handles, one per user.
//   2. Users subscribe by asking the ChatRoom for a handle (Sender)
//      and a Receiver. They publish through the ChatRoom; they
//      receive on their own Receiver.
//   3. The ChatRoom's publish() fans the event to every user's
//      Sender.

use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone, Debug)]
pub struct Event {
    pub from: String,
    pub text: String,
}

pub struct ChatRoom {
    subscribers: Vec<Sender<Event>>,
}

impl ChatRoom {
    pub fn new() -> Self { Self { subscribers: Vec::new() } }

    /// Subscribe a user; they get back a Receiver to read events on.
    pub fn join(&mut self) -> Receiver<Event> {
        let (tx, rx) = mpsc::channel();
        self.subscribers.push(tx);
        rx
    }

    /// Publish an event to every subscriber. Broken channels (a user
    /// who dropped their Receiver) are pruned.
    pub fn publish(&mut self, ev: Event) {
        self.subscribers.retain(|s| s.send(ev.clone()).is_ok());
    }
}

impl Default for ChatRoom { fn default() -> Self { Self::new() } }

// ---- Sugar: a User struct that hides the channel plumbing ----------

pub struct User {
    pub name: String,
    rx: Receiver<Event>,
}

impl User {
    pub fn join(room: &mut ChatRoom, name: impl Into<String>) -> Self {
        let rx = room.join();
        Self { name: name.into(), rx }
    }

    /// Publish a message to the room.
    pub fn say(&self, room: &mut ChatRoom, text: impl Into<String>) {
        room.publish(Event { from: self.name.clone(), text: text.into() });
    }

    /// Drain every pending event so we can report what the user saw.
    pub fn inbox(&self) -> Vec<Event> {
        self.rx.try_iter().collect()
    }
}

fn main() {
    let mut room = ChatRoom::new();

    // Users join (the ChatRoom hands each a Receiver).
    let alice = User::join(&mut room, "Alice");
    let bob   = User::join(&mut room, "Bob");
    let carol = User::join(&mut room, "Carol");

    // Each publish fans out to every current subscriber.
    alice.say(&mut room, "hello everyone");
    bob.say(&mut room, "hey Alice");

    // Each user's inbox contains the messages the ROOM routed to them.
    for user in [&alice, &bob, &carol] {
        println!("--- {} inbox ---", user.name);
        for ev in user.inbox() {
            println!("  {} > {}", ev.from, ev.text);
        }
    }
}
