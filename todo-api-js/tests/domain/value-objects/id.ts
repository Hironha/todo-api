import { describe, test, expect } from "bun:test";

import { Id, InvalidId } from "@domain/value-objects/id";

describe("Id methods", () => {
    test("Create a new Id", () => {
        const id = Id.create();

        expect(id).toBeInstanceOf(Id);
        expect(id.value).toBeString();
    });

    test("Parse a string into Id", () => {
        const id = Id.create();
        const parsed = Id.parse(id.value);

        expect(parsed.isOk()).toBeTrue();
        expect(parsed.ok()?.value === id.value).toBeTrue();
    });

    test("Compare same Id correctly", () => {
        const id = Id.create();
        const parsed = Id.parse(id.value);

        expect(parsed.ok()?.eq(id)).toBeTrue();
        expect(parsed.ok()?.neq(id)).toBeFalse();
    });

    test("Fail parsing strings that are not uuid v4", () => {
        const src = "not-v4";
        const parsed = Id.parse(src);

        expect(parsed.isErr()).toBeTrue();
        expect(parsed.err()).toBeInstanceOf(InvalidId);
    });
});
