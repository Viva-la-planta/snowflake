use snowflake::generator::Pool;

#[test]
fn test_pool() {
    let mut pool = Pool::new(3);
    let snowflake = pool.generate();

    assert_eq!(snowflake.get_worker_id(), 3);


    println!("{:?}", snowflake.get_id());
    println!("{:?}", snowflake);
}