import { v4 as uuidv4, validate, version } from "uuid";

import { ValueObject } from "@domain/value-objects";
import { Exception } from "../utils/exception";
import { Ok, Err, type Result } from "../utils/result";

/**
 * A wrapper around a random generated value that can be used
 * as a data identifier
 */
export class Id extends ValueObject<Id> {
    readonly value: string;

    protected constructor(value: string) {
        super();
        this.value = value;
    }

    /**
     * Create a new instance of {@link Id}
     */
    static create(): Id {
        return new Id(uuidv4());
    }

    /**
     * Attempt to parse a string into a new instance of {@link Id}
     */
    static parse(value: string): Result<Id, InvalidId> {
        const valid = validate(value) && version(value) === 4;
        if (!valid) {
            return new Err(new InvalidId(value));
        }
        return new Ok(new Id(value));
    }

    eq(rhs: Id): boolean {
        return this.value === rhs.value;
    }
}

export class InvalidId extends Exception<"InvalidId"> {
    constructor(id: string) {
        const message = `Value ${id} is not an valid Id`;
        super("InvalidId", message);
    }
}
