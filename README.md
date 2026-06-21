# spin v4 internal services broken repro

## usage

```bash
spin up --build &
curl http://127.0.0.1:3000
```

Observe the error that results:

```
❯ spin up --build
Building component one with `cargo build --target wasm32-wasip2 --release`
Working directory: "./one"
    Finished `release` profile [optimized] target(s) in 0.06s
Building component spin4-internal-broken with `cargo build --target wasm32-wasip2 --release`
    Finished `release` profile [optimized] target(s) in 0.09s
Building component two with `cargo build --target wasm32-wasip2 --release`
Working directory: "./two"
    Finished `release` profile [optimized] target(s) in 0.06s
Finished building all Spin components
Logging component stdio to ".spin/logs/"

Serving http://127.0.0.1:3000
Available Routes:
  spin4-internal-broken: http://127.0.0.1:3000 (wildcard)
Handling request to Some("http://127.0.0.1:3000/")
2026-06-21T03:41:42.142564Z ERROR spin_trigger_http::wasip3: Component error handling request: ErrorCode::DnsError(DnsErrorPayload { rcode: Some("address not available"), info-code: Some(0) })
2026-06-21T03:41:42.142594Z ERROR spin_trigger_http::server: Error processing request: oneshot canceled
```
