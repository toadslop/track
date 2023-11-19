use criterion::{criterion_group, criterion_main, Criterion};
use util::ActixAsyncRuntime;
use utilities::spawn::spawn_app;

mod util;

pub fn criterion_benchmark(c: &mut Criterion) {
    let runtime = ActixAsyncRuntime(actix_web::rt::Runtime::new().unwrap());
    let handle = runtime.spawn(async { spawn_app().await.unwrap() });
    let test_app = runtime.block_on(handle).unwrap();

    c.bench_function("health_check_perf", |b| {
        b.to_async(&runtime).iter(|| test_app.health_check());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
