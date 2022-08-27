use snowflake::Snowflake;

#[test]
fn test_pool() {
    let test_flake_id: u64 = 1100578097115137;
    let snf = Snowflake::init_from(1100578097115137, 262398266, 3, 30283, 1);

    let generated = Snowflake::new(test_flake_id);

    assert_eq!(generated.get_timestamp_at(), snf.get_timestamp_at());
}

#[test]
fn test_id() {
    let snowflake: Snowflake = "1100578097115137".into();
    let generated = Snowflake::new(1100578097115137);

    assert_eq!(snowflake.get_timestamp_at(), generated.get_timestamp_at());
}