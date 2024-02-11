import { type UseCase } from "@domain/use-case";
import { type RemoveTodoInput, type RemoveTodoOutput } from "@application/dtos/todo/remove";
import { type RemoveTodoRequest, type RemoveTodoPresenter } from "@adapters/dtos/todo/remove";

export class RemoveTodoController<TPresenter extends RemoveTodoPresenter<any>> {
    constructor(
        private interactor: UseCase<RemoveTodoInput, RemoveTodoOutput>,
        private presenter: TPresenter
    ) {}

    async run(req: RemoveTodoRequest): Promise<ReturnType<TPresenter["present"]>> {
        const input = req.parse();
        if (input.isErr()) {
            return this.presenter.present(input);
        }

        const result = await this.interactor.exec(input.value);
        return this.presenter.present(result);
    }
}
