// Chain of Responsibility — middleware around a core service.
//
// Two idiomatic Rust shapes:
//
//   A) `Vec<Box<dyn Handler>>` — simple, dynamic at runtime. Good
//      when middleware is read from config or composed at startup
//      based on feature flags.
//
//   B) Nested generic wrappers — the Tower/axum "Layer" pattern.
//      Each layer is generic over the next, so the whole chain
//      monomorphizes to a single inlined function. No vtable.
//
// This file shows both, applied to a tiny pretend HTTP service.

// ---- Common types ---------------------------------------------------

#[derive(Debug, Clone)]
pub struct Request {
    pub path: String,
    pub headers: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct Response { pub status: u16, pub body: String }

type ChainResult = Result<Response, Response>;

// ---- A) Vec<Box<dyn Handler>> ---------------------------------------

pub trait Handler: Send + Sync {
    fn call(&self, req: &Request, next: &Next) -> ChainResult;
}

pub struct Next<'a> {
    chain: &'a [Box<dyn Handler>],
    service: &'a dyn Fn(&Request) -> Response,
}

impl<'a> Next<'a> {
    pub fn call(&self, req: &Request) -> ChainResult {
        match self.chain.split_first() {
            Some((first, rest)) => first.call(req, &Next { chain: rest, service: self.service }),
            None => Ok((self.service)(req)),
        }
    }
}

pub struct LoggingHandler;
impl Handler for LoggingHandler {
    fn call(&self, req: &Request, next: &Next) -> ChainResult {
        println!("[log] → {} {}", "GET", req.path);
        let res = next.call(req);
        match &res {
            Ok(r)  => println!("[log] ← {}", r.status),
            Err(r) => println!("[log] ← {} (short-circuit)", r.status),
        }
        res
    }
}

pub struct AuthHandler;
impl Handler for AuthHandler {
    fn call(&self, req: &Request, next: &Next) -> ChainResult {
        let authed = req.headers.iter().any(|(k, v)| k == "x-auth" && v == "ok");
        if !authed {
            return Err(Response { status: 401, body: "unauthorized".into() });
        }
        next.call(req)
    }
}

pub struct RateLimit { pub max: usize }
impl Handler for RateLimit {
    fn call(&self, req: &Request, next: &Next) -> ChainResult {
        if req.path.len() > self.max {
            return Err(Response { status: 429, body: "too long".into() });
        }
        next.call(req)
    }
}

pub fn run_with_chain(
    req: &Request,
    chain: &[Box<dyn Handler>],
    service: &dyn Fn(&Request) -> Response,
) -> ChainResult {
    Next { chain, service }.call(req)
}

// ---- B) Nested generic layers — static dispatch ---------------------

pub trait Service {
    fn call(&self, req: &Request) -> Response;
}

pub struct CoreService;
impl Service for CoreService {
    fn call(&self, req: &Request) -> Response {
        Response { status: 200, body: format!("hello from {}", req.path) }
    }
}

pub struct Logging<S> { pub inner: S }
impl<S: Service> Service for Logging<S> {
    fn call(&self, req: &Request) -> Response {
        println!("[log2] -> {}", req.path);
        let r = self.inner.call(req);
        println!("[log2] <- {}", r.status);
        r
    }
}

pub struct Auth<S> { pub inner: S }
impl<S: Service> Service for Auth<S> {
    fn call(&self, req: &Request) -> Response {
        let ok = req.headers.iter().any(|(k, v)| k == "x-auth" && v == "ok");
        if ok {
            self.inner.call(req)
        } else {
            Response { status: 401, body: "unauthorized".into() }
        }
    }
}

fn main() {
    let core: Box<dyn Fn(&Request) -> Response> = Box::new(|req| Response {
        status: 200, body: format!("ok: {}", req.path),
    });

    // A) Runtime chain built from a Vec.
    let chain: Vec<Box<dyn Handler>> = vec![
        Box::new(LoggingHandler),
        Box::new(AuthHandler),
        Box::new(RateLimit { max: 40 }),
    ];

    let req_ok = Request {
        path: "/hello".into(),
        headers: vec![("x-auth".into(), "ok".into())],
    };
    let req_bad = Request {
        path: "/hello".into(),
        headers: vec![],
    };
    println!("--- ok ---");
    let _ = run_with_chain(&req_ok, &chain, &*core);
    println!("--- unauth ---");
    let _ = run_with_chain(&req_bad, &chain, &*core);

    // B) Static nested chain — fully inlined.
    let svc = Logging { inner: Auth { inner: CoreService } };
    println!("--- static ok ---");
    let r = svc.call(&req_ok);
    println!("resp {} {}", r.status, r.body);
}
