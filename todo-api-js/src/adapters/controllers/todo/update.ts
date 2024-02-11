import { type UseCase } from "@domain/use-case";
import { type UpdateTodoInput, type UpdateTodoOutput } from "@application/dtos/todo/update";
import { type UpdateTodoRequest, type UpdateTodoPresenter } from "@adapters/dtos/todo/update";

export class UpdateTodoController<TPresenter extends UpdateTodoPresenter<any>> {
    constructor(
        private interactor: UseCase<UpdateTodoInput, UpdateTodoOutput>,
        private presenter: TPresenter
    ) {}

    async run(req: UpdateTodoRequest): Promise<ReturnType<TPresenter["present"]>> {
        const input = req.parse();
        if (input.isErr()) {
            return this.presenter.present(input);
        }

        const result = await this.interactor.exec(input.value);
        return this.presenter.present(result);
    }
}
