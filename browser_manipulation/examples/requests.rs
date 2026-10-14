//! Every response a page gets in its first seconds, by bytes, with some URL parts blocked:
//! what a block leaves through. `cargo r --example requests -- <profile> <url> <secs> [blocked part]...`

use std::time::Duration;

use browser_manipulation::{Browser, Launch, Robot, Viewport};
use futures::StreamExt as _;

#[tokio::main]
async fn main() {
	let mut args = std::env::args().skip(1);
	let profile = args.next().expect("profile").into();
	let url = args.next().expect("url");
	let secs: u64 = args.next().expect("secs").parse().unwrap();
	let blocked: Vec<String> = args.collect();
	let launch = Launch::Owned {
		profile,
		executable: std::env::var("BM_TEST_CHROME").expect("set by the devShell").into(),
		headless: true,
		viewport: Some(Viewport {
			width: 1280,
			height: 900,
			device_scale_factor: 1.,
		}),
		args: vec![],
	};
	let browser = Browser::launch(launch, Robot, None).await.unwrap();
	let mut tab = browser.tab().await.unwrap();
	tab.block(&blocked.iter().map(String::as_str).collect::<Vec<_>>()).await.unwrap();
	let mut seen = std::pin::pin!(tab.responses("").await.unwrap());
	tab.goto(&url).await.unwrap();
	let mut got = Vec::new();
	let _ = tokio::time::timeout(Duration::from_secs(secs), async {
		while let Some(r) = seen.next().await {
			if let Ok(r) = r {
				got.push((r.body.len(), r.url));
			}
		}
	})
	.await; // the window is the point: the stream never ends by itself
	let sw: String = tab
		.eval(
			"String(navigator.serviceWorker && navigator.serviceWorker.controller && navigator.serviceWorker.controller.scriptURL)",
			(),
		)
		.await
		.unwrap();
	println!("service worker: {sw}");
	got.sort();
	for (len, url) in got.iter().rev().take(40) {
		println!("{len:>9} {}", &url[..url.len().min(160)]);
	}
	println!("{} responses, {} bytes", got.len(), got.iter().map(|g| g.0).sum::<usize>());
	drop(tab);
	browser.close().await.unwrap();
}
