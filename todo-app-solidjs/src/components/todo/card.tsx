import { Show } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { Typography } from "../ui/typography";
import { classes } from "../../core/utils/classes";
import { type TodoStatus } from "../../core/entities/todo";
import { unreachable } from "../../core/utils/unreachable";

export type TodoCardProps = {
  title: string;
  description?: string;
  status: TodoStatus;
};

export function TodoCard(props: TodoCardProps): JSX.Element {
  const statusConfig = getStatusConfig(props.status);
  const statusStyles = (): string => {
    return classes("badge badge-outline my-2 whitespace-nowrap").add(statusConfig.color).build();
  };

  return (
    <div class="card shadow-xl">
      <div class="card-body">
        <div class="flex gap-3 justify-between">
          <Typography.Title level={2}>{props.title}</Typography.Title>
          <div class={statusStyles()}>
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
