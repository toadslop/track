use criterion::{criterion_group, criterion_main, Criterion};
use util::ActixAsyncRuntime;
use utilities::{dummy::gen_dummy_user, spawn::spawn_app};

mod util;

pub fn criterion_benchmark(c: &mut Criterion) {
    let runtime = ActixAsyncRuntime(actix_web::rt::Runtime::new().unwrap());
    let handle = runtime.spawn(async { spawn_app().await.unwrap() });
    let test_app = runtime.block_on(handle).unwrap();

    c.bench_function("health_check", |b| {
        b.to_async(&runtime).iter(|| test_app.health_check());
    });

    let runtime = ActixAsyncRuntime(actix_web::rt::Runtime::new().unwrap());
    let handle = runtime.spawn(async { spawn_app().await.unwrap() });
    let test_app = runtime.block_on(handle).unwrap();

    c.bench_function("signup", |b| {
        b.to_async(&runtime)
            .iter(|| test_app.signup_owned(gen_dummy_user()));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
