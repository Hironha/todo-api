import { splitProps, Show } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../core/utils/classes";
import { unreachable } from "../../core/utils/unreachable";
import { Typography } from "./typography";

export type FieldProps = JSX.LabelHTMLAttributes<HTMLLabelElement> & {
  label?: string;
  error?: string;
};

export function Field(props: FieldProps): JSX.Element {
  const [local, labelProps] = splitProps(props, ["label", "error", "class"]);
  const labelStyles = (): string => {
    return classes("form-control w-full").add(local.class).build();
  };

  return (
    <label {...labelProps} class={labelStyles()}>
      <Show when={local.label}>
        {(label) => (
          <div class="label">
            <span class="label-text">{label()}</span>
          </div>
        )}
      </Show>

      {props.children}

      <Show when={local.error}>
        {(error) => (
          <div class="label">
            <Typography.Text class="label-text-alt text-error" size="sm">
              {error()}
            </Typography.Text>
          </div>
        )}
      </Show>
    </label>
  );
}

export type InputSize = "sm" | "md" | "lg";
export type InputStatus = "ok" | "err";
export type InputProps = JSX.InputHTMLAttributes<HTMLInputElement> & {
  status?: InputStatus;
  /** @default "md" */
  size?: InputSize;
  onChange?: JSX.ChangeEventHandler<HTMLInputElement, Event>;
};

export function Input(props: InputProps): JSX.Element {
  const [local, inputProps] = splitProps(props, ["status", "size", "class"]);
  const styles = (): string => {
    return classes("input input-bordered w-full")
      .add(local.status && getStatusStyle(local.status))
      .add(getSizeStyle(local.size ?? "md"))
      .add("focus-within:border-primary focus-within:outline-none")
      .add(local.class)
      .build();
  };

  return <input {...inputProps} class={styles()} />;
}

export type SelectProps = JSX.SelectHTMLAttributes<HTMLSelectElement> & {
  status?: InputStatus;
  /** @default "md" */
  size?: InputSize;
};

export function Select(props: SelectProps): JSX.Element {
  const [local, selectProps] = splitProps(props, ["status", "size", "class"]);
  const styles = (): string => {
    return classes("select select-bordered w-full")
      .add(local.status && getStatusStyle(local.status))
      .add(getSizeStyle(local.size ?? "md"))
      .add("focus-within:border-primary focus-within:outline-none")
      .add(local.class)
      .build();
  };

  return <select {...selectProps} class={styles()} />;
}

function getStatusStyle(status: InputStatus): string {
  switch (status) {
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
