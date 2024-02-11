import { Id } from "@domain/value-objects/id";
import { Ok, Err, type Result } from "@domain/utils/result";
import { ValueObject } from "@domain/value-objects";
import { Entity } from ".";

export type TodoEntityProps = {
    id: Id;
    title: Title;
    description?: Description | undefined;
    status: Status;
    todoAt?: Date | undefined;
    createdAt: Date;
    updatedAt: Date;
};

export type CreateTodoEntityProps = {
    title: Title;
    description?: Description | undefined;
    status: Status;
    todoAt?: Date | undefined;
};

export class TodoEntity extends Entity<TodoEntityProps> {
    static create(props: CreateTodoEntityProps): TodoEntity {
        const now = new Date();
        return new TodoEntity({ ...props, id: Id.create(), createdAt: now, updatedAt: now });
    }

    static from(props: Readonly<TodoEntityProps>): TodoEntity {
        return new TodoEntity(props);
    }

    get id(): Id {
        return this.props.id;
    }

    get title(): Title {
        return this.props.title;
    }
}

export class Title extends ValueObject<Title> {
    static readonly MIN_LENGTH = 3;
    static readonly MAX_LENGTH = 64;

    private constructor(public readonly value: string) {
        super();
    }

    static parse(title: string): Result<Title, string> {
        if (title.length > Title.MAX_LENGTH) {
            const message = `Todo title cannot have more than ${Title.MAX_LENGTH} characters`;
            return new Err(message);
        }

        if (title.length < Title.MIN_LENGTH) {
            const message = `Todo title must have at least ${Title.MIN_LENGTH} characters`;
            return new Err(message);
        }

        return new Ok(new Title(title));
    }

    eq(rhs: Title): boolean {
        return this.value === rhs.value;
    }
}

export class Description extends ValueObject<Description> {
    static readonly MAX_LENGTH = 128;

    private constructor(public readonly value: string) {
        super();
        this.value = value;
    }

    static parse(description: string): Result<Description, string> {
        if (description.length > this.MAX_LENGTH) {
            const message = `Todo description cannot have more than ${this.MAX_LENGTH} characters`;
            return new Err(message);
        }
        return new Ok(new Description(description));
    }

    eq(rhs: Description): boolean {
        return this.value === rhs.value;
    }
}

export type StatusKind = "todo" | "in_progress" | "done";

export class Status extends ValueObject<Status> {
    static readonly OPTIONS: StatusKind[] = ["todo", "in_progress", "done"];

    constructor(public readonly value: StatusKind) {
        super();
    }

    static parse(value: string): Result<Status, string> {
        if (!Status.OPTIONS.includes(value as StatusKind)) {
            const options = Status.OPTIONS.join(", ");
            const message = `Todo status must be one of the following options: ${options}`;
            return new Err(message);
        }
        return new Ok(new Status(value as StatusKind));
    }

    eq(rhs: Status): boolean {
        return this.value === rhs.value;
    }
}
