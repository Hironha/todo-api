import { type UseCase } from "@domain/use-case";
import { type CreateTodoInput, type CreateTodoOutput } from "@application/dtos/todo/create";
import { type CreateTodoRequest, type CreateTodoPresenter } from "@adapters/dtos/todo/create";

export class CreateTodoController<TPresenter extends CreateTodoPresenter<any>> {
    constructor(
        private interactor: UseCase<CreateTodoInput, CreateTodoOutput>,
        private presenter: TPresenter
    ) {}

    async run(req: CreateTodoRequest): Promise<ReturnType<TPresenter["present"]>> {
        const input = req.parse();
        if (input.isErr()) {
            return this.presenter.present(input);
        }

        const result = await this.interactor.exec(input.value);
        return this.presenter.present(result);
    }
}
