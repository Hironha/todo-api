import { For, Show, Switch, Match, createResource } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "./components/ui/typography";
import { Content } from "./components/ui/content";
import { Empty } from "./components/ui/empty";
import { TodoCard } from "./components/todo/card";
import { TodoForm, type FormController, TodoFormValues } from "./components/todo/form";
import { Modal, type ModalRef } from "./components/ui/modal";
import { useThemeConfig } from "./hooks/ui/theme";
import {
  createTodo as createTodoService,
  listTodos as listTodoService,
} from "./core/services/todo";
import { useMutation } from "./hooks/useMutation";
import { formatDateYmd, getDateFromConventionalDate } from "./core/utils/date";

const CREATE_TODO_FORM_ID = "create_todo_form";

export default function App() {
  let createModalRef: ModalRef | undefined;
  const [todoList, todoListActions] = createResource(listTodoService);
  const createTodo = useMutation(createTodoService);

  const submitCreateTodoForm = async (
    form: FormController,
    values: TodoFormValues
  ): Promise<void> => {
    const todoAt = values.todoAt
      ? formatDateYmd(getDateFromConventionalDate(values.todoAt))
      : undefined;

    const response = await createTodo.mutate({
      title: values.title,
      status: values.status,
      description: values.description,
      todoAt,
    });

    if (response.isOk()) {
      form.reset();
      todoListActions.refetch();
    } else {
      console.log("TODO: handle error");
    }
  };

  return (
    <Content class="flex flex-col gap-2 justify-center">
      <MainActions onCreateClick={() => createModalRef?.show()} />

      <Switch fallback={<Typography.Text>Failed loading todos :(</Typography.Text>}>
        <Match when={todoList.loading}>
          <div class="w-full p-8 flex justify-center items-center">
            <span class="text-primary loading loading-spinner loading-lg" />
          </div>
        </Match>

        <Match when={todoList()?.ok()}>
          {(todos) => (
            <Show
              when={todos().data.length > 0}
              fallback={<Empty class="my-4" message="Ainda não há nenhum item cadastrado." />}
            >
              <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                <For each={todos().data}>
                  {(todo) => (
                    <TodoCard
                      title={todo.title}
                      status={todo.status}
                      description={todo.description}
                    />
                  )}
                </For>
              </div>
            </Show>
          )}
        </Match>

        <Match when={todoList()?.err()}>
          {(error) => <Typography.Text>{error()}</Typography.Text>}
        </Match>
      </Switch>

      <Modal ref={createModalRef} id="create_todo_modal" title="Criar Todo">
        <TodoForm id={CREATE_TODO_FORM_ID} onSubmit={submitCreateTodoForm} />

        <div class="flex gap-3 justify-between mt-6">
          <button
            form={CREATE_TODO_FORM_ID}
            type="reset"
            class="btn btn-primary btn-outline btn-sm"
            disabled={createTodo.loading()}
            onClick={() => createModalRef?.close()}
          >
            Cancelar
          </button>

          <input
            class="btn btn-primary btn-sm"
            type="submit"
            form={CREATE_TODO_FORM_ID}
            disabled={createTodo.loading()}
            value="Criar"
          />
        </div>
      </Modal>
    </Content>
  );
}

type MainActionsProps = {
  onCreateClick: () => void;
};

function MainActions(props: MainActionsProps): JSX.Element {
  const themeConfig = useThemeConfig();

  const setLightTheme = (): void => themeConfig.set("light");
  const setDarkTheme = (): void => themeConfig.set("dark");

  return (
    <div class="flex space-between gap-4 mb-4">
      <button class="btn btn-primary btn-sm" onClick={props.onCreateClick}>
        Novo
      </button>

      <div class="flex gap-4 ml-auto">
        <button class="btn btn-primary btn-sm" onClick={setLightTheme}>
          Tema claro
        </button>
        <button class="btn btn-primary btn-sm" onClick={setDarkTheme}>
          Tema escuro
        </button>
      </div>
    </div>
  );
}
