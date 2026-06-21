# Spin v4 issues reproduction repo

Issues demonstrated with this repo:

1. Spin v4 does not route *.spin.internal HTTP requests
2. `spin new -E akamai-functions` breaks `spin build`
3. Akamai Functions fails to deploy applications with two components

## 1. Spin v4 does not route *.spin.internal HTTP requests

```bash
❯ spin up --build &
❯ curl http://127.0.0.1:3000
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

## 2. `spin new -E akamai-functions` breaks `spin build`

Edit `spin.toml` to uncomment this line:

```toml
targets = ["akamai-functions"]
```

Try to build, and observe the error:

```bash
❯ spin build
Building component two with `cargo build --target wasm32-wasip2 --release`
Working directory: "./two"
    Finished `release` profile [optimized] target(s) in 0.06s
Building component spin4-internal-broken with `cargo build --target wasm32-wasip2 --release`
    Finished `release` profile [optimized] target(s) in 0.10s
Building component one with `cargo build --target wasm32-wasip2 --release`
Working directory: "./one"
    Finished `release` profile [optimized] target(s) in 0.06s
Finished building all Spin components
Error: unable to check if the application is compatible with deployment targets

Caused by:
   0: Failed to get akamai:functions@1.0.0 from registry
   1: registry error: Authentication failure: {"errors":[{"code":"DENIED","message":"requested access to the resource is denied"}]}

   2: Authentication failure: {"errors":[{"code":"DENIED","message":"requested access to the resource is denied"}]}
```

## 3. Akamai Functions fails to deploy applications with two components

```bash
❯ spin build
[...]

❯ spin aka deploy
Name of new app: spin4-internal-broken
Creating new app spin4-internal-broken in account indirect
Note: If you would instead like to deploy to an existing app, cancel this deploy and link this workspace to the app with `spin aka app link`
OK to continue? yes
Workspace linked to app spin4-internal-broken
Error: failed to wait for deployment to go live

Caused by:
   Deployment did not go live within 60 seconds
   
❯ spin aka app status
Name: spin4-internal-broken
ID: 3f00b11f-3ec8-4833-b865-d66e283d9c09
URLs:
  https://3f00b11f-3ec8-4833-b865-d66e283d9c09.fwf.app
  https://3f00b11f-3ec8-4833-b865-d66e283d9c09.aka.fermyon.tech (deprecated)
Created at: 2026-06-21 05:09:27 UTC
Invocations: app has not been used in the last 7 days

❯ curl https://3f00b11f-3ec8-4833-b865-d66e283d9c09.fwf.app
Unknown snapshot
```
