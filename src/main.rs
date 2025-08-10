#[macro_use]
extern crate alloc;

mod sketch;

use async_std::task::block_on;
use sketch::run_app;

fn main() {
    block_on(async {
        run_app(0, 0).await;
    });
}