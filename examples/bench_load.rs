use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

async fn run_bench(client: reqwest::Client, url: &str, concurrency: usize, duration_secs: u64) {
    let ok_count = Arc::new(AtomicU64::new(0));
    let err_count = Arc::new(AtomicU64::new(0));
    let latencies = Arc::new(tokio::sync::Mutex::new(Vec::<u64>::with_capacity(100_000)));
    let deadline = Instant::now() + Duration::from_secs(duration_secs);

    let mut handles = Vec::with_capacity(concurrency);
    for _ in 0..concurrency {
        let client = client.clone();
        let url = url.to_string();
        let ok_count = ok_count.clone();
        let err_count = err_count.clone();
        let latencies = latencies.clone();
        handles.push(tokio::spawn(async move {
            let mut local_lats = Vec::with_capacity(4096);
            while Instant::now() < deadline {
                let t0 = Instant::now();
                match client.get(&url).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        let _ = resp.bytes().await;
                        ok_count.fetch_add(1, Ordering::Relaxed);
                        local_lats.push(t0.elapsed().as_micros() as u64);
                    }
                    _ => {
                        err_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            let mut guard = latencies.lock().await;
            guard.extend(local_lats);
        }));
    }

    let start = Instant::now();
    for h in handles {
        let _ = h.await;
    }
    let elapsed = start.elapsed().as_secs_f64();

    let ok = ok_count.load(Ordering::Relaxed);
    let err = err_count.load(Ordering::Relaxed);
    let rps = (ok as f64) / elapsed;

    let mut lats = latencies.lock().await;
    lats.sort_unstable();
    let p50 = lats.get(lats.len() / 2).copied().unwrap_or(0) as f64 / 1000.0;
    let p99 = lats
        .get((lats.len() * 99) / 100)
        .copied()
        .unwrap_or(0) as f64
        / 1000.0;

    println!(
        "{:<12} | {:>10.1} RPS | ok={:<7} err={:<3} | p50={:>5.2}ms p99={:>6.2}ms",
        url.rsplit('/').next().unwrap_or(url),
        rps,
        ok,
        err,
        p50,
        p99
    );
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(128)
        .build()
        .expect("Failed to build reqwest client");

    let concurrency = 64;
    let duration = 5;

    // Warmup
    let _ = client.get("http://127.0.0.1:8080/plaintext").send().await;
    let _ = client.get("http://127.0.0.1:8080/db-users").send().await;

    println!("Concurrency: {} | Duration: {}s per endpoint", concurrency, duration);
    run_bench(client.clone(), "http://127.0.0.1:8080/plaintext", concurrency, duration).await;
    run_bench(client.clone(), "http://127.0.0.1:8080/json", concurrency, duration).await;
    run_bench(client, "http://127.0.0.1:8080/db-users", concurrency, duration).await;
}
