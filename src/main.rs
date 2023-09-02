pub mod snowflake;
pub mod generator;

pub use self::snowflake::Snowflake;

use std::{time::Duration, thread};

use generator::Pool;

fn main() {
    let mut pool = Pool::new(3);
    thread::sleep(Duration::from_millis(200));
    let mut pool2 = Pool::new(3);

    let batch_one = (
        pool.generate(),
        pool.generate(),
        pool.generate()
    );

    let batch_two = (
        pool2.generate(),
        pool2.generate(),
        pool2.generate()
    );

    println!("{:?}", batch_one.0.get_id());
    println!("{:?}", batch_one.1.get_id());
    println!("{:?}", batch_one.2.get_id());

    println!("{:?}", batch_two.0.get_id());
    println!("{:?}", batch_two.1.get_id());
    println!("{:?}", batch_two.2.get_id());
}