import { For, Show } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "./components/ui/typography";
import { Content } from "./components/ui/content";
import { unreachable } from "./core/utils/unreachable";
import { type TodoStatus, type Todo } from "./core/entities/todo";
import { useThemeConfig } from "./core/hooks/theme";
import { classes } from "./core/utils/classes";

const items: Todo[] = [
  {
    id: "id1",
    title: "Melhorar qualidade da UI",
    description: "A UI está bem cru ainda e precisa de algumas melhorias no design",
    status: "done",
    todoAt: "2023-02-12",
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  },
  {
    id: "id2",
    title: "Melhorar qualidade da API",
    description: "O endpoint de criação de todo poderia aceitar tags talvez?",
    status: "in_progress",
    todoAt: "2023-02-12",
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  },
  {
    id: "id3",
    title: "Melhorar qualidade do código",
    status: "todo",
    todoAt: "2023-02-12",
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  },
];

export default function App() {
  const themeConfig = useThemeConfig();

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

      <div class="grid grid-cols-3 gap-4">
        <For each={items} fallback={<div>Loading...</div>}>
          {(todo) => (
            <TodoCard title={todo.title} status={todo.status} description={todo.description} />
          )}
        </For>
      </div>
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
