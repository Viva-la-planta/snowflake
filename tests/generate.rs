use std::{time::Duration, thread};

use snowflake::generator::Pool;

#[test]
fn test_pool() {
    let mut pool = Pool::new(3);
    let snowflake = pool.generate();

    assert_eq!(snowflake.get_worker_id(), 3);


    println!("{:?}", snowflake.get_id());
    println!("{:?}", snowflake);
}

#[test]
fn unique_generations() {
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

    assert_ne!(batch_one.0.get_id(), batch_two.0.get_id());
    assert_ne!(batch_one.1.get_id(), batch_two.1.get_id());
    assert_ne!(batch_one.2.get_id(), batch_two.2.get_id());

}