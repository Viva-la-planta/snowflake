export const VIVA_EPOCH: bigint = 1660769388000n;
export const WORKER_ID_BITS: bigint = 5n;
export const PROCESS_ID_BITS: bigint = 5n;
export const INCREMENT_BITS: bigint = 12n;
export const MAX_WORKER_ID: bigint = 1n << WORKER_ID_BITS;
export const MAX_PROCESS_ID: bigint = 1n << PROCESS_ID_BITS;
export const MAX_INCREMENT_ID: bigint = 1n << INCREMENT_BITS;
export const MAX_TIMESTAMP: bigint = 1n << (64n - WORKER_ID_BITS - PROCESS_ID_BITS - INCREMENT_BITS);

export class Snowflake {
    public id!: bigint;
    private timestamp!: bigint;
    private workerId!: bigint;
    private datacenterId!: bigint;
    private incrementId!: bigint;

    constructor(snowflake: bigint) {
        this.from_id(snowflake);
    }

    public static from_string(snowflake: string) {
        return new Snowflake(BigInt(snowflake));
    }

    public static from_id(id: bigint) {
        return new Snowflake(id);
    }

    public static create(workerId: number, datacenterId: number, incrementId: number) {
        const timestamp: bigint = (BigInt(Date.now()) - VIVA_EPOCH) % MAX_TIMESTAMP;

        return new this(Snowflake.generate_id(timestamp, workerId, datacenterId, incrementId));
    }

    public static generate_id(timestamp: bigint, workerId: number, datacenterId: number, incrementId: number): bigint {
        let id = timestamp << WORKER_ID_BITS << PROCESS_ID_BITS << INCREMENT_BITS;
        id |= (BigInt(workerId) % MAX_WORKER_ID) << PROCESS_ID_BITS << INCREMENT_BITS;
        id |= (BigInt(datacenterId) % MAX_PROCESS_ID) << INCREMENT_BITS;
        id |= BigInt(incrementId) % MAX_INCREMENT_ID;
        return id;
    }

    public from_id(id: bigint) {
        if (id < 0) {
            throw new Error(`Invalid snowflake id ${id}`);
        }

        if ((id + 1n) === 1n) {
            throw new Error(`Invalid snowflake id ${id}`);
        }

        // timestamp
        const timestamp = id >> (WORKER_ID_BITS + PROCESS_ID_BITS + INCREMENT_BITS);
        // worker id
        const workerId = (id >> (PROCESS_ID_BITS + INCREMENT_BITS)) & ((1n << WORKER_ID_BITS) - 1n);
        // datacenter id
        const datacenterId = (id >> INCREMENT_BITS) & ((1n << PROCESS_ID_BITS) - 1n);
        // increment id
        const incrementId = id & ((1n << INCREMENT_BITS) - 1n);

        this.id = id;
        this.timestamp = timestamp;
        this.workerId = workerId;
        this.datacenterId = datacenterId;
        this.incrementId = incrementId;
    }

    // getters for the snowflake parts
    public get_timestamp(): bigint {
        return this.timestamp;
    }

    public get_worker_id(): bigint {
        return this.workerId;
    }

    public get_process_id(): bigint {
        return this.datacenterId;
    }

    public get_increment_id(): bigint {
        return this.incrementId;
    }

    public get_id(): bigint {
        return this.id;
    }
}