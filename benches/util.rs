use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ActixAsyncRuntime(pub actix_web::rt::Runtime);

impl Deref for ActixAsyncRuntime {
    type Target = actix_web::rt::Runtime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ActixAsyncRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl criterion::async_executor::AsyncExecutor for ActixAsyncRuntime {
    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        self.0.block_on(future)
    }
}

impl criterion::async_executor::AsyncExecutor for &ActixAsyncRuntime {
    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        self.0.block_on(future)
    }
}
