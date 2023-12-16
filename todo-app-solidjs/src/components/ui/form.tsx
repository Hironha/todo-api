import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../core/utils/classes";
import { unreachable } from "../../core/utils/unreachable";
import { Show } from "solid-js";
import { Typography } from "./typography";

export type FieldProps = JSX.LabelHTMLAttributes<HTMLLabelElement> & {
  label?: string;
  error?: string
};

export function Field(props: FieldProps): JSX.Element {
  const { label, error, class: styles, ...labelProps } = props;
  const labelStyles = classes("form-control w-full").add(styles).build();

  return (
    <label {...labelProps} class={labelStyles}>
      <Show when={label}>
        {(label) => (
          <div class="label">
            <span class="label-text">{label()}</span>
          </div>
        )}
      </Show>

      {props.children}

      <Show when={error}>
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
export type InputStatus = 'ok' | 'err'
export type InputProps = JSX.InputHTMLAttributes<HTMLInputElement> & {
  status?: InputStatus;
  /** @default "md" */
  size?: InputSize;
  onChange?: JSX.ChangeEventHandler<HTMLInputElement, Event>;
};

export function Input(props: InputProps): JSX.Element {
  const { status, size, class: styles, ...inputProps } = props;
  const inputStyles = classes("input input-bordered w-full")
    .add(status && getStatusStyle(status))
    .add(getSizeStyle(size ?? "md"))
    .add("focus-within:border-primary focus-within:outline-none")
    .add(styles)
    .build();

  return <input {...inputProps} class={inputStyles} />;
}

export type SelectProps = JSX.SelectHTMLAttributes<HTMLSelectElement> & {
  status?: InputStatus;
  /** @default "md" */
  size?: InputSize;
};

export function Select(props: SelectProps): JSX.Element {
  const { status, size, class: styles, ...inputProps } = props;
  const inputStyles = classes("select select-bordered w-full")
    .add(status && getStatusStyle(status))
    .add(getSizeStyle(size ?? "md"))
    .add("focus-within:border-primary focus-within:outline-none")
    .add(styles)
    .build();

  return <select {...inputProps} class={inputStyles} />;
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
