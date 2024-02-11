import { Err } from "@domain/utils/result";
import { InternalError } from "@domain/utils/exception";
import { type UseCase } from "@domain/use-case";
import { type TodoRepository } from "@application/repositories/todo";
import { type RemoveTodoOutput, type RemoveTodoInput } from "@application/dtos/todo/remove";

export class RemoveTodoUseCase implements UseCase<RemoveTodoInput, RemoveTodoOutput> {
    constructor(private repository: TodoRepository) {}

    async exec(input: RemoveTodoInput): Promise<RemoveTodoOutput> {
        try {
            return this.repository.deleteById(input.id);
        } catch (e) {
            const error = new InternalError(e instanceof Error ? e : undefined);
            return new Err(error);
        }
    }
}
