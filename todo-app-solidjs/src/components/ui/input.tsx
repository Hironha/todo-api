import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../core/utils/classes";
import { unreachable } from "../../core/utils/unreachable";
import { Show } from "solid-js";
import { Typography } from "./typography";

export type InputSize = "sm" | "md" | "lg";
export type InputStatus = { kind: "ok" } | { kind: "err"; message: string };

export type InputProps = {
  name: string;
  status?: InputStatus;
  label?: string;
  class?: string;
  /** @default "text" */
  type?: string;
  placeholder?: string;
  /** @default "md" */
  size?: InputSize;
};

export function Input(props: InputProps): JSX.Element {
  const labelStyles = classes("form-control w-full")
    .add(props.class)
    .build();

  const inputStyles = classes("input input-bordered w-full")
    .add(props.status && getStatusStyle(props.status))
    .add(getSizeStyle(props.size ?? "md"))
    .add("focus-within:border-primary focus-within:outline-none")
    .build();

  return (
    <label class={labelStyles} for={props.name}>
      <Show when={props.label}>
        {(label) => (
          <div class="label">
            <span class="label-text">{label()}</span>
          </div>
        )}
      </Show>

      <input
        class={inputStyles}
        type={props.type ?? "text"}
        placeholder={props.placeholder}
        name={props.name}
      />

      <div class="label">
        <Show when={props.status?.kind === "err" ? props.status : false}>
          {(error) => (
            <Typography.Text class="label-text-alt text-error" size="sm">
              {error().message}
            </Typography.Text>
          )}
        </Show>
      </div>
    </label>
  );
}

function getStatusStyle(status: InputStatus): string {
  switch (status.kind) {
    case "ok":
      return "border-success";
    case "err":
      return "border-error";
    default:
      return unreachable(status);
  }
}

function getSizeStyle(size: InputSize): string {
  switch (size) {
    case "sm":
      return "input-sm";
    case "md":
      return "input-md";
    case "lg":
      return "input-lg";
    default:
      return unreachable(size);
  }
}
