import { Description, Status, Title, TodoEntity } from "@domain/entities/todo";
import { InternalError } from "@domain/utils/exception";
import { Err, Ok, type Result } from "@domain/utils/result";
import { Id } from "@domain/value-objects/id";

export type TodoModel = {
    id: string;
    title: string;
    description?: string;
    status: TodoStatus;
    todoAt?: string;
    createdAt: string;
    updatedAt: string;
};

export type TodoStatus = "todo" | "in_progress" | "done";

export function createFromEntity(entity: TodoEntity): TodoModel {
    const values = entity.unpack();
    return {
        id: values.id.value,
        title: values.title.value,
        description: values.description?.value,
        status: values.status.value,
        todoAt: values.todoAt?.toISOString(),
        createdAt: values.createdAt.toISOString(),
        updatedAt: values.updatedAt.toISOString(),
    };
}

export function mapToEntity(model: TodoModel): Result<TodoEntity, InternalError> {
    try {
        const entity = TodoEntity.from({
            id: Id.parse(model.id).expect("Invalid todo model id"),
            title: Title.parse(model.title).expect("Invalid todo model title"),
            description: model.description
                ? Description.parse(model.description).expect("Invalid todo model description")
                : undefined,
            status: Status.parse(model.status).expect("Invalid todo model status"),
            todoAt: model.todoAt ? new Date(model.todoAt) : undefined,
            createdAt: new Date(model.createdAt),
            updatedAt: new Date(model.updatedAt),
        });

        return new Ok(entity);
    } catch (e) {
        return new Err(new InternalError(e));
    }
}
