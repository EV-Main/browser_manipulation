# browser_manipulation
![Minimum Supported Rust Version](https://img.shields.io/badge/nightly-2026--06--29+-ab6000.svg)
![Newest Supported Nightly](https://img.shields.io/badge/nightly-2026--09--28-ab6000.svg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/browser_manipulation.svg?color=fc8d62&logo=rust" height="20" style=flat-square>](https://crates.io/crates/browser_manipulation)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs&style=flat-square" height="20">](https://docs.rs/browser_manipulation)
![Lines Of Code](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/browser_manipulation-loc.json)
![Build time](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/browser_manipulation-build.json)
![Test time](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/browser_manipulation-test.json)
<br>
[![errors](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/browser_manipulation-errors.json)](https://github.com/EV-Main/browser_manipulation/actions/workflows/errors.yml?query=branch%3Amain)
[![warnings](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/browser_manipulation-warnings.json)](https://github.com/EV-Main/browser_manipulation/actions/workflows/warnings.yml?query=branch%3Amain)

One Playwright-protocol browser framework for every consumer: patchright driver (no `Runtime.enable`), human-shaped `Motion`, capture-on-failure.
<!-- markdownlint-disable -->
<details>
<summary>
<h2>Installation</h2>
</summary>

Needs the `patchright-core` driver at `playwright_rs::PLAYWRIGHT_VERSION`: consume this flake's `packages.patchright` and set `PLAYWRIGHT_CLI_JS=${patchright}/package/cli.js`, `PLAYWRIGHT_NODE_EXE=${nodejs}/bin/node`, `PLAYWRIGHT_SKIP_DRIVER_DOWNLOAD=1`. The headless Chromium that driver was released against is `packages.headless-shell` (`${headless-shell}/bin/chrome-headless-shell`, Linux).


</details>
<!-- markdownlint-restore -->

## Usage
```rust
let browser = Browser::launch(Launch::Owned { profile, executable, headless: true, viewport: None, args: vec![] }, Robot, Some(artifacts)).await?;
let mut tab = browser.tab().await?;
tab.goto("https://example.com").await?;
tab.click("text=More").await?; // on failure: Error.capture holds PNG + HTML
```



<br>

#### Dev links
- <sup><a href="./docs/ARCHITECTURE.md">architecture</a></sup>
- <sup><a href="https://github.com/EV-Main/.github/blob/main/code/STYLE.md">style</a></sup>
- <sup><a href="https://github.com/EV-Main/.github/tree/main/code/conventions">conventions</a></sup>
- <sup><a href="https://github.com/EV-Main/.github/blob/main/code/conventions/git">contributing</a></sup>

#### License

<sup>
	Licensed under <a href="LICENSE">Blue Oak 1.0.0</a>
</sup>

<br>

<sub>
	Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be licensed as above, without any additional terms or conditions.
</sub>

