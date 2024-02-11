import { InternalError } from "@domain/utils/exception";
import { Err } from "@domain/utils/result";
import { TodoEntity } from "@domain/entities/todo";
import { type UseCase } from "@domain/use-case";
import { type CreateTodoInput, type CreateTodoOutput } from "@application/dtos/todo/create";
import { type TodoRepository } from "@application/repositories/todo";

export class CreateTodoUseCase implements UseCase<CreateTodoInput, CreateTodoOutput> {
    constructor(private repository: TodoRepository) {}

    async exec(input: CreateTodoInput): Promise<CreateTodoOutput> {
        try {
            const entity = TodoEntity.create({
                title: input.title,
                description: input.description,
                status: input.status,
                todoAt: input.todoAt,
            });

            const result = await this.repository.create(entity);
            return result.map(() => entity);
        } catch (e) {
            const error = new InternalError(e instanceof Error ? e : undefined);
            return new Err(error);
        }
    }
}
