import { For, Show, Switch, Match, createResource } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "./components/ui/typography";
import { Content } from "./components/ui/content";
import { Empty } from "./components/ui/empty";
import { TodoCard } from "./components/todo/card";
import { TodoForm } from "./components/todo/form";
import { Modal, type ModalRef } from "./components/ui/modal";
import { useThemeConfig } from "./hooks/ui/theme";
import { TodoService } from "./core/services/todo";

export default function App() {
  let createModalRef: ModalRef | undefined = undefined;
  const [todoResource] = createResource(TodoService.list);

  return (
    <Content class="flex flex-col gap-2 justify-center">
      <MainActions onCreateClick={() => createModalRef?.show()} />

      <Switch fallback={<Typography.Text>Failed loading todos :(</Typography.Text>}>
        <Match when={todoResource.loading}>
          <div class="w-full p-8 flex justify-center items-center">
            <span class="text-primary loading loading-spinner loading-lg" />
          </div>
        </Match>

        <Match when={todoResource()?.ok()}>
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

        <Match when={todoResource()?.err()}>
          {(error) => <Typography.Text>{error()}</Typography.Text>}
        </Match>
      </Switch>

      <Modal ref={createModalRef} id="create_todo_modal" title="Criar Todo">
        <TodoForm id="create_todo_form" onSubmit={console.log} />

        <div class="flex gap-3 justify-between mt-6">
          <button
            class="btn btn-primary btn-outline btn-sm"
            onClick={() => createModalRef?.close()}
          >
            Cancelar
          </button>

          <input
            class="btn btn-primary btn-sm"
            type="submit"
            form="create_todo_form"
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
