# Architecture
One framework crate every browser-driving consumer goes through. It speaks the Playwright protocol (`playwright-rs`) to the `patchright-core` driver, which never sends `Runtime.enable` nor flags automation.

```
consumer ──▶ Browser<M: Motion> ──tab()──▶ Tab<'_, M> ──every action──┐
   │            │ launch(Launch, M, Option<Artifacts>)                  ▼
   │            │ Launch::Owned{profile,executable,headless,viewport,args} M: pause → path/keys/wheel → playwright-rs
   │            │ Launch::Attach{cdp}   (someone else's Chrome)          │ Err
   │            └ ProfileLock                                            ▼
   │                                                              capture: full PNG(+tEXt) + HTML → Artifacts
   └──────────── Error{kind, capture} (paths rendered in the diagnostic)
```

- `browser.rs` — launch/attach, driver assertion, profile lock. A `Viewport` carries its `device_scale_factor`: the driver takes one only with a viewport. `args` go after the driver's own switches; Chromium keeps only the last `--disable-features=`/`--enable-features=`, so ours are merged with the list the pinned driver's bundle spells. Headless, the headed UA is probed by a throwaway launch once per executable (path and mtime) and kept in the profile.
- `tab.rs` — the whole action surface; each action is paced and shaped by `Motion`, and captured on failure. Beside the actions: `responses` (bodies of what the page fetches), `route` (the page's own requests, sent with a rewritten body), `cookies`/`wait_for_cookie` (what a request to a given URL would carry, asked of the page's target, since an attached Chrome files every profile's pages under one context), `block` (matching URLs aborted through the driver's routing, which also reaches the page's workers and service worker; `Network.setBlockedURLs` reaches neither). `element_shot` is a CDP clip at its own `scale`, measured on a settled page and returned with that `Geometry`; a scrolling ancestor that clips the element grows the viewport for that shot.
- `motion.rs` — `Motion` trait; `Robot` (driver-native, instant) and `Noise` (human-shaped).
- `capture.rs` — `Artifacts` directory with retention; PNG provenance.
- `error.rs` — `Error`.

Pacing across a session (active hours, caps, bursts) is not browser work and lives with the consumer; `Motion` only shapes single actions.

Invariants: [spec/invariants.md](spec/invariants.md).
