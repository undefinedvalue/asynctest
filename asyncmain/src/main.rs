#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

fn main() {
    #[allow(unused_must_use)]
    asynclib::SomeTrait::doasync(&mut Foo);
}

struct Foo;

impl asynclib::SomeTrait for Foo {
    async fn doasync(&mut self) {
    }
}