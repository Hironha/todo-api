import { type UseCase } from "@domain/use-case";
import { type FindTodoInput, type FindTodoOutput } from "@application/dtos/todo/find";
import { type FindTodoPresenter, type FindTodoRequest } from "@adapters/dtos/todo/find";

export class FindTodoController<TPresenter extends FindTodoPresenter<any>> {
    constructor(
        private interactor: UseCase<FindTodoInput, FindTodoOutput>,
        private presenter: TPresenter
    ) {}

    async run(req: FindTodoRequest): Promise<ReturnType<TPresenter["present"]>> {
        const input = req.parse();
        if (input.isErr()) {
            return this.presenter.present(input);
        }

        const result = await this.interactor.exec(input.value);
        return this.presenter.present(result);
    }
}
