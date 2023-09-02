const Generator = require("../src/Generator").Generator;

test("worker id matches", () => {
    let generator = new Generator(2);

    expect(generator.generate().get_worker_id()).toBe(2n);
});

test("datacenter id matches", () => {
    let generator = new Generator(2, 3);

    expect(generator.generate().get_process_id()).toBe(3n);
});

test("generated ids are unique", () => {
    const generator1 = new Generator(1);
    const generator2 = new Generator(1, Date.now() + 200);

    const ids = []

    for (let i = 0; i < 100; i++) {
        ids.push(generator1.generate().get_id());
        ids.push(generator2.generate().get_id());
    }

    expect(ids.length).toBe(200);

    // make sure the ids are unique
    for (let i = 0; i < ids.length; i++) {
        let p = ids.shift();
        if (ids.includes(p)) {
            throw new Error("duplicate id found");
        }
    }
});