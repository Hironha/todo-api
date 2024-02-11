import { type UseCase } from "@domain/use-case";
import { type ListTodosInput, type ListTodosOutput } from "@application/dtos/todo/list";
import { type ListTodosRequest, type ListTodosPresenter } from "@adapters/dtos/todo/list";

export class ListTodosController<TPresenter extends ListTodosPresenter<any>> {
    constructor(
        private interactor: UseCase<ListTodosInput, ListTodosOutput>,
        private presenter: TPresenter
    ) {}

    async run(req: ListTodosRequest): Promise<ReturnType<TPresenter["present"]>> {
        const input = req.parse();
        if (input.isErr()) {
            return this.presenter.present(input);
        }

        const result = await this.interactor.exec(input.value);
        return this.presenter.present(result);
    }
}
