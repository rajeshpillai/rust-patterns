// Broken example — two common Mediator footguns.
// This file is expected to FAIL to compile.
//
//   1. Each User holds a &mut Vec<User> reference so it can mutate
//      the whole list on send. Multiple Users wanting shared mutable
//      access to the room is precisely what the borrow checker
//      forbids.
//
//   2. Users hold references to each OTHER (peer-to-peer), which is
//      the anti-pattern Mediator was invented to avoid. In Rust the
//      lifetimes bite too — circular &T references are either
//      impossible to set up, or require Rc + RefCell + Weak, which
//      signals the design is wrong.

pub struct User<'a> {
    pub name: String,
    // Each user directly references all the others. This is exactly
    // the mess Mediator exists to prevent — and in Rust the borrow
    // checker makes it painful to even construct.
    pub peers: Vec<&'a User<'a>>,
}

impl<'a> User<'a> {
    pub fn broadcast(&self, text: &str) {
        for peer in &self.peers {
            // Immutable references can't call mutating methods — and
            // a mutating method on a peer would need &mut, which we
            // can't get from our &Vec.
            peer.receive(text);
            //^^^^^^^^^^^ error[E0596]: cannot borrow `*peer` as
            //            mutable, as it is behind a `&` reference
        }
    }
    pub fn receive(&mut self, _text: &str) {}
}

pub fn set_up() {
    let mut a = User { name: "A".into(), peers: vec![] };
    let mut b = User { name: "B".into(), peers: vec![] };

    // Constructing the peer-to-peer mesh: A points at B, B points at A.
    // The borrow-checker error varies by how you write it, but you
    // cannot have both immutable-refs-into-each-other AND mutable
    // methods on each — you're fighting aliasing-XOR-mutation.
    a.peers.push(&b);
    b.peers.push(&a);
    //          ^^ error[E0502]: cannot borrow `a` as immutable
    //             because it is also borrowed as mutable

    a.broadcast("hi");
}

fn main() { set_up(); }
