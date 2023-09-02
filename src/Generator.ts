import { Snowflake } from "./Snowflake";

export class Generator {
    private workerId: number;
    private datacenterId: number;
    private increment: number;

    public constructor(workerId: number, datacenterId?: number) {
        this.workerId = workerId;
        this.datacenterId = datacenterId || Date.now();
        this.increment = 0;
    }

    public generate(): Snowflake {
        return Snowflake.create(this.workerId, this.datacenterId, ++this.increment);
    }
}