"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Generator = void 0;
const Snowflake_1 = require("./Snowflake");
class Generator {
    workerId;
    datacenterId;
    increment;
    constructor(workerId, datacenterId) {
        this.workerId = workerId;
        this.datacenterId = datacenterId || Date.now();
        this.increment = 0;
    }
    generate() {
        return Snowflake_1.Snowflake.create(this.workerId, this.datacenterId, ++this.increment);
    }
}
exports.Generator = Generator;
