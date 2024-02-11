import { InternalError } from "@domain/utils/exception";
import { Err } from "@domain/utils/result";
import { type UseCase } from "@domain/use-case";
import { type UpdateTodoInput, type UpdateTodoOutput } from "@application/dtos/todo/update";
import { type TodoRepository } from "@application/repositories/todo";

export class UpdateTodoUseCase implements UseCase<UpdateTodoInput, UpdateTodoOutput> {
    constructor(private repository: TodoRepository) {}

    async exec(input: UpdateTodoInput): Promise<UpdateTodoOutput> {
        try {
            return this.repository.update(input);
        } catch (e) {
            const error = new InternalError(e instanceof Error ? e : undefined);
            return new Err(error);
        }
    }
}
