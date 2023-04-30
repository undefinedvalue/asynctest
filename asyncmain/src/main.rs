#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

fn main() {
    asynclib::SomeTrait::doasync(&mut Foo);
}

struct Foo;

impl asynclib::SomeTrait for Foo {
    async fn doasync(&mut self) {
    }
}