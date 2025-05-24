use criterion::{black_box, criterion_group, criterion_main, Criterion};
use est::collections::MapExt;
use est::future::FutureExt;
use std::collections::HashMap;
use std::future::ready;

fn bench_replace_key(c: &mut Criterion) {
    c.bench_function("hashmap_replace_key", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            map.insert("key1".to_string(), 42);
            map.insert("key2".to_string(), 84);
            
            // Test successful replacement
            black_box(map.replace_key("key1", "key3".to_string()).unwrap());
            
            // Test error cases
            black_box(map.replace_key("nonexistent", "key4".to_string()).unwrap_err());
            black_box(map.replace_key("key3", "key2".to_string()).unwrap_err());
        })
    });
}

fn bench_with_cancel_signal(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("with_cancel_signal_unpin", |b| {
        b.iter(|| {
            rt.block_on(async {
                let future = ready(42);
                let cancel = ready(());
                black_box(future.with_cancel_signal_unpin(cancel).await)
            })
        })
    });
    
    c.bench_function("with_cancel_signal_boxed", |b| {
        b.iter(|| {
            rt.block_on(async {
                let future = Box::pin(ready(42));
                let cancel = Box::pin(ready(()));
                black_box(future.with_cancel_signal(cancel).await)
            })
        })
    });
}

criterion_group!(benches, bench_replace_key, bench_with_cancel_signal);
criterion_main!(benches);