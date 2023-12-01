import { Show } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "./components/ui/typography";
import { Content } from "./components/ui/content";
import { useThemeConfig } from "./core/hooks/theme";

export default function App() {
  const themeConfig = useThemeConfig();

  const setLightTheme = (): void => themeConfig.set("light");
  const setDarkTheme = (): void => themeConfig.set("dark");

  return (
    <Content class="flex flex-col gap-2 justify-center">
      <Typography.Title level={1}>Todos</Typography.Title>

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
        <TodoCard
          title="Teste"
          status="todo"
          description="Apenas um teste, pode excluir depois, ou você acredito que isso irá funcionar?"
        />

        <TodoCard
          title="Teste"
          status="in-progress"
          description="Apenas um teste, pode excluir depois, ou você acredito que isso irá funcionar?"
        />

        <TodoCard
          title="Teste"
          status="in-progress"
          description="Apenas um teste, pode excluir depois, ou você acredito que isso irá funcionar?"
        />

        <TodoCard
          title="Teste"
          status="in-progress"
          description="Apenas um teste, pode excluir depois, ou você acredito que isso irá funcionar?"
        />
      </div>
    </Content>
  );
}

type TodoCardProps = {
  status: string;
  title: string;
  description?: string;
};

function TodoCard(props: TodoCardProps): JSX.Element {
  return (
    <div class="card w-96 shadow-xl">
      <div class="card-body">
        <div class="flex gap-2 justify-between">
          <Typography.Title level={2}>{props.title}</Typography.Title>
          <div class="badge badge-outline">{props.status}</div>
        </div>

        <Show when={props.description}>
          {(description) => <Typography>{description()}</Typography>}
        </Show>
      </div>
    </div>
  );
}
