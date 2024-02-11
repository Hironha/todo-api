import { Err } from "@domain/utils/result";
import { InternalError } from "@domain/utils/exception";
import { type UseCase } from "@domain/use-case";
import { type TodoRepository } from "@application/repositories/todo";
import { type ListTodosInput, type ListTodosOutput } from "@application/dtos/todo/list";

export class ListTodosUseCase implements UseCase<ListTodosInput, ListTodosOutput> {
    constructor(private repository: TodoRepository) {}

    async exec(input: ListTodosInput): Promise<ListTodosOutput> {
        try {
            return this.repository.listPaginated(input);
        } catch (e) {
            const error = new InternalError(e instanceof Error ? e : undefined);
            return new Err(error);
        }
    }
}
