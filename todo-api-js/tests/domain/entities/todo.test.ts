import { expect, test, describe } from "bun:test";

import { Id } from "@domain/value-objects/id";
import { TodoEntity, Title, Description, Status, type StatusKind } from "@domain/entities/todo";

describe("Todo title parsing", () => {
    test("Fail because string does not have enough length", () => {
        const title = "as";
        const result = Title.parse(title);

        expect(title.length < Title.MIN_LENGTH);
        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeString();
        expect(result.err()).toBeString();
    });

    test("Fail because string it too big", () => {
        const title =
            "This is a really big string to test if Title will return an error while parsing it";
        const result = Title.parse(title);

        expect(title.length > Title.MAX_LENGTH);
        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeString();
        expect(result.err()).toBeString();
    });

    test("Success for strings with length in allowed range", () => {
        const title = "Normal title";
        const result = Title.parse(title);

        expect(title.length >= Title.MIN_LENGTH && title.length <= Title.MAX_LENGTH);
        expect(result.isOk()).toBeTrue();
        expect(result.value).toBeInstanceOf(Title);
        expect(result.ok()).toBeInstanceOf(Title);
    });
});

describe("Todo title comparison", () => {
    test("Success comparing different titles", () => {
        const left = Title.parse("Left title").ok()!;
        const right = Title.parse("Right title").ok()!;

        expect(left).toBeDefined();
        expect(right).toBeDefined();
        expect(left.eq(right) || right.eq(left)).toBeFalse();
        expect(left.neq(right) && right.neq(left)).toBeTrue();
    });

    test("Success comparing same title", () => {
        const title = "Some title";
        const left = Title.parse(title).ok()!;
        const right = Title.parse(title).ok()!;

        expect(left).toBeDefined();
        expect(right).toBeDefined();
        expect(left.eq(right) && right.eq(left)).toBeTrue();
        expect(left.neq(right) || right.neq(left)).toBeFalse();
    });
});

describe("Todo description parsing", () => {
    test("Fail because string is too big", () => {
        const base =
            "This is a really big description, so to make it even bigger, it'll be doubled";
        const description = `${base}. ${base}`;
        const result = Description.parse(description);

        expect(description.length > Description.MAX_LENGTH);
        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeString();
        expect(result.err()).toBeString();
    });

    test("Success for string with length in allowed range", () => {
        const description = "This is a normal description";
        const result = Description.parse(description);

        expect(description.length < Description.MAX_LENGTH);
        expect(result.isOk()).toBeTrue();
        expect(result.value).toBeInstanceOf(Description);
        expect(result.ok()).toBeInstanceOf(Description);
    });
});

describe("Todo description comparison", () => {
    test("Success comparing different descriptions", () => {
        const left = Description.parse("Left description").ok()!;
        const right = Description.parse("Right description").ok()!;

        expect(left).toBeDefined();
        expect(right).toBeDefined();
        expect(left.eq(right) || right.eq(left)).toBeFalse();
        expect(left.neq(right) && right.neq(left)).toBeTrue();
    });

    test("Success comparing same descriptions", () => {
        const description = "Description test";
        const left = Description.parse(description).ok()!;
        const right = Description.parse(description).ok()!;

        expect(left).toBeDefined();
        expect(right).toBeDefined();
        expect(left.eq(right) && right.eq(left)).toBeTrue();
        expect(left.neq(right) || right.neq(left)).toBeFalse();
    });
});

describe("Todo status parsing", () => {
    test("Fail because string is not a status option", () => {
        const status = "not-valid";
        const result = Status.parse(status);

        expect(Status.OPTIONS.includes(status as StatusKind)).toBeFalse();
        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeString();
        expect(result.err()).toBeString();
    });

    test("Success parsing status", () => {
        const statuses = Status.OPTIONS;
        const results = statuses.map(Status.parse);

        expect(results.length).toEqual(Status.OPTIONS.length);
        for (const result of results) {
            expect(result.isOk()).toBeTrue();
            expect(result.value).toBeInstanceOf(Status);
            expect(result.ok()).toBeInstanceOf(Status);
        }
    });
});

describe("Todo status comparison", () => {
    test("Success comparing different statuses", () => {
        const left: StatusKind = "done";
        const leftStatus = Status.parse(left).ok()!;
        const right: StatusKind = "todo";
        const rightStatus = Status.parse(right).ok()!;

        expect(leftStatus).toBeDefined();
        expect(rightStatus).toBeDefined();
        expect(leftStatus.eq(rightStatus) || rightStatus.eq(leftStatus)).toBeFalse();
        expect(leftStatus.neq(rightStatus) && rightStatus.neq(leftStatus)).toBeTrue();
    });

    test("Success comparing same statuses", () => {
        const left: StatusKind = "in_progress";
        const leftStatus = Status.parse(left).ok()!;
        const right: StatusKind = "in_progress";
        const rightStatus = Status.parse(right).ok()!;

        expect(leftStatus).toBeDefined();
        expect(rightStatus).toBeDefined();
        expect(leftStatus.eq(rightStatus) && rightStatus.eq(leftStatus)).toBeTrue();
        expect(leftStatus.neq(rightStatus) || rightStatus.neq(leftStatus)).toBeFalse();
    });
});

describe("Todo entity behaviors", () => {
    test("Success on creation with all fields", () => {
        const entity = TodoEntity.create({
            title: Title.parse("Title").ok()!,
            description: Description.parse("Description").ok()!,
            status: Status.parse("done").ok()!,
            todoAt: new Date(),
        });
        const props = entity.unpack();

        expect(props.id).toBeInstanceOf(Id);
        expect(props.title).toBeInstanceOf(Title);
        expect(props.description).toBeInstanceOf(Description);
        expect(props.status).toBeInstanceOf(Status);
        expect(props.todoAt).toBeInstanceOf(Date);
        expect(props.createdAt).toBeInstanceOf(Date);
        expect(props.updatedAt).toBeInstanceOf(Date);
    });
});
