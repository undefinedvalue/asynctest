#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

pub trait SomeTrait {
    async fn doasync(&mut self);
}