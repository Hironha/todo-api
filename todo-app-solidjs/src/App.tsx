import { For, Show, Switch, Match, createResource } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "./components/ui/typography";
import { Content } from "./components/ui/content";
import { unreachable } from "./core/utils/unreachable";
import { type TodoStatus } from "./core/entities/todo";
import { useThemeConfig } from "./hooks/ui/theme";
import { classes } from "./core/utils/classes";
import { TodoService } from "./core/services/todo";

export default function App() {
  const themeConfig = useThemeConfig();
  const [todoResource] = createResource(TodoService.list);

  const setLightTheme = (): void => themeConfig.set("light");
  const setDarkTheme = (): void => themeConfig.set("dark");

  return (
    <Content class="flex flex-col gap-2 justify-center">
      <div class="flex space-between gap-4 mb-4">
        <button class="btn btn-primary btn-sm">Novo</button>

        <div class="flex gap-4 ml-auto">
          <button class="btn btn-primary btn-sm" onClick={setLightTheme}>
            Tema claro
          </button>
          <button class="btn btn-primary btn-sm" onClick={setDarkTheme}>
            Tema escuro
          </button>
        </div>
      </div>

      <Switch fallback={<Typography.Text>Failed loading todos :(</Typography.Text>}>
        <Match when={todoResource.loading}>
          <div class="w-full p-8 flex justify-center items-center">
            <span class="text-primary loading loading-spinner loading-lg" />
          </div>
        </Match>

        <Match when={todoResource()?.ok()}>
          {(todos) => (
            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              <For each={todos().value.data} fallback={<div>Loading...</div>}>
                {(todo) => (
                  <TodoCard
                    title={todo.title}
                    status={todo.status}
                    description={todo.description}
                  />
                )}
              </For>
            </div>
          )}
        </Match>

        <Match when={todoResource()?.err()}>
          {(error) => <Typography.Text>{error().value}</Typography.Text>}
        </Match>
      </Switch>
    </Content>
  );
}

type TodoCardProps = {
  title: string;
  description?: string;
  status: TodoStatus;
};

function TodoCard(props: TodoCardProps): JSX.Element {
  const statusConfig = getStatusConfig(props.status);
  const statusStyles = classes("badge badge-outline my-2 whitespace-nowrap")
    .add(statusConfig.color)
    .build();

  return (
    <div class="card shadow-xl">
      <div class="card-body">
        <div class="flex gap-3 justify-between">
          <Typography.Title level={2}>{props.title}</Typography.Title>
          <div class={statusStyles}>
            <Typography.Text size="sm">{statusConfig.label}</Typography.Text>
          </div>
        </div>

        <Show when={props.description}>
          {(description) => <Typography>{description()}</Typography>}
        </Show>
      </div>
    </div>
  );
}

function getStatusConfig(status: TodoStatus): { color: string; label: string } {
  switch (status) {
    case "todo":
      return { color: "text-sky-600", label: "A fazer" };
    case "in_progress":
      return { color: "text-yellow-600", label: "Em progresso" };
    case "done":
      return { color: "text-green-600", label: "Feito" };
    default:
      return unreachable(status);
  }
}
