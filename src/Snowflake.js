"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Snowflake = exports.MAX_TIMESTAMP = exports.MAX_INCREMENT_ID = exports.MAX_PROCESS_ID = exports.MAX_WORKER_ID = exports.INCREMENT_BITS = exports.PROCESS_ID_BITS = exports.WORKER_ID_BITS = exports.VIVA_EPOCH = void 0;
exports.VIVA_EPOCH = 1660769388000n;
exports.WORKER_ID_BITS = 5n;
exports.PROCESS_ID_BITS = 5n;
exports.INCREMENT_BITS = 12n;
exports.MAX_WORKER_ID = 1n << exports.WORKER_ID_BITS;
exports.MAX_PROCESS_ID = 1n << exports.PROCESS_ID_BITS;
exports.MAX_INCREMENT_ID = 1n << exports.INCREMENT_BITS;
exports.MAX_TIMESTAMP = 1n << (64n - exports.WORKER_ID_BITS - exports.PROCESS_ID_BITS - exports.INCREMENT_BITS);
class Snowflake {
    id;
    timestamp;
    workerId;
    datacenterId;
    incrementId;
    constructor(snowflake) {
        this.from_id(snowflake);
    }
    static from_string(snowflake) {
        return new Snowflake(BigInt(snowflake));
    }
    static from_id(id) {
        return new Snowflake(id);
    }
    static create(workerId, datacenterId, incrementId) {
        const timestamp = (BigInt(Date.now()) - exports.VIVA_EPOCH) % exports.MAX_TIMESTAMP;
        return new this(Snowflake.generate_id(timestamp, workerId, datacenterId, incrementId));
    }
    static generate_id(timestamp, workerId, datacenterId, incrementId) {
        let id = timestamp << exports.WORKER_ID_BITS << exports.PROCESS_ID_BITS << exports.INCREMENT_BITS;
        id |= (BigInt(workerId) % exports.MAX_WORKER_ID) << exports.PROCESS_ID_BITS << exports.INCREMENT_BITS;
        id |= (BigInt(datacenterId) % exports.MAX_PROCESS_ID) << exports.INCREMENT_BITS;
        id |= BigInt(incrementId) % exports.MAX_INCREMENT_ID;
        return id;
    }
    from_id(id) {
        if (id < 0) {
            throw new Error(`Invalid snowflake id ${id}`);
        }
        if ((id + 1n) === 1n) {
            throw new Error(`Invalid snowflake id ${id}`);
        }
        // timestamp
        const timestamp = id >> (exports.WORKER_ID_BITS + exports.PROCESS_ID_BITS + exports.INCREMENT_BITS);
        // worker id
        const workerId = (id >> (exports.PROCESS_ID_BITS + exports.INCREMENT_BITS)) & ((1n << exports.WORKER_ID_BITS) - 1n);
        // datacenter id
        const datacenterId = (id >> exports.INCREMENT_BITS) & ((1n << exports.PROCESS_ID_BITS) - 1n);
        // increment id
        const incrementId = id & ((1n << exports.INCREMENT_BITS) - 1n);
        this.id = id;
        this.timestamp = timestamp;
        this.workerId = workerId;
        this.datacenterId = datacenterId;
        this.incrementId = incrementId;
    }
    // getters for the snowflake parts
    get_timestamp() {
        return this.timestamp;
    }
    get_worker_id() {
        return this.workerId;
    }
    get_process_id() {
        return this.datacenterId;
    }
    get_increment_id() {
        return this.incrementId;
    }
    get_id() {
        return this.id;
    }
}
exports.Snowflake = Snowflake;
