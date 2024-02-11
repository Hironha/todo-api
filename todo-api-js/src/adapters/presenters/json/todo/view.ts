import { type TodoEntity } from "@domain/entities/todo";

// View does not need behaviors or access restriction
// so it's fine to just be a type
export type TodoView = {
    id: string;
    title: string;
    description?: string | undefined;
    status: string;
    todoAt?: string | undefined;
    /** Date and time in ISO 8601 UTC format */
    createdAt: string;
    /** Date and time in ISO 8601 UTC format */
    updatedAt: string;
};

export function createViewFromEntity(entity: TodoEntity): TodoView {
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
