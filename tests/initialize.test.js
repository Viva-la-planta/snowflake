const { Snowflake } = require('../src/Snowflake');

test("intialization matches expected values", () => {
    let id = 138095515882668084n;

    let snowflake = new Snowflake(id);

    expect(snowflake.get_id()).toBe(id);
});