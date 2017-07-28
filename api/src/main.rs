extern crate futures;
extern crate tokio_timer;
extern crate futures_cpupool;

extern crate r2d2;
extern crate r2d2_postgres;

use std::time::Duration;
use std::io;

use futures_cpupool::{CpuPool, CpuFuture};
use futures::Future;
use tokio_timer::Timer;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};

fn main() {
    let pool = CpuPool::new_num_cpus();

    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new("postgres://postgres@postgres", TlsMode::None).unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();

    // let futur: CpuFuture<bool, &'static str> = pool.spawn_fn(move || {
    //     let timer = Timer::default();
    //     timer.sleep(Duration::from_millis(4000))
    //         .then(|_| Ok(false))
    // });
    //
    // match futur.wait() {
    //     Ok(true) => println!("Done, true"),
    //     Ok(false) => println!("Done, false"),
    //     Err(_) => println!("Fail"),
    // };


}
