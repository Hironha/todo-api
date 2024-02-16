import { type TodoEntity } from "@domain/entities/todo";
import { type JsonView } from "../view";

// View does not need behaviors or access restriction
// so it's fine to just be a type
export type TodoJsonView = JsonView<{
    id: string;
    title: string;
    description?: string | undefined;
    status: string;
    todoAt?: string | undefined;
    /** Date and time in ISO 8601 UTC format */
    createdAt: string;
    /** Date and time in ISO 8601 UTC format */
    updatedAt: string;
}>;

export function fromEntity(entity: TodoEntity): TodoJsonView {
    const props = entity.unpack();
    return {
        id: props.id.value,
        title: props.title.value,
        description: props.description?.value,
        status: props.status.value,
        // TODO: adjust date format
        todoAt: props.todoAt?.toISOString(),
        createdAt: props.createdAt.toISOString(),
        updatedAt: props.updatedAt.toISOString(),
    };
}
