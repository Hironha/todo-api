import { Err } from "@domain/utils/result";
import { InternalError } from "@domain/utils/exception";
import { type UseCase } from "@domain/use-case";
import { type TodoRepository } from "@application/repositories/todo";
import { type FindTodoInput, type FindTodoOutput } from "@application/dtos/todo/find";

export class FindTodoUseCase implements UseCase<FindTodoInput, FindTodoOutput> {
    constructor(private repository: TodoRepository) {}

    async exec(input: FindTodoInput): Promise<FindTodoOutput> {
        try {
            return this.repository.findById(input.id);
        } catch (e) {
            const error = new InternalError(e instanceof Error ? e : undefined);
            return new Err(error);
        }
    }
}
