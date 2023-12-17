import { type JSX } from "solid-js/jsx-runtime";
import { createForm, reset } from "@modular-forms/solid";

import { Input, Field, Select } from "../ui/form";
import {
  parseTodoTitle,
  parseTodoStatus,
  type TodoTitle,
  type TodoStatus,
} from "../../core/entities/todo";
import {
  formatConventionalDate,
  isConventionalDate,
  type ConventionalDate,
} from "../../core/utils/date";
import { classes } from "../../core/utils/classes";
import { unreachable } from "../../core/utils/unreachable";

export type TodoFormValues = {
  title: TodoTitle;
  status: TodoStatus;
  description?: string;
  todoAt?: ConventionalDate;
};

export type FormController = { reset: () => void };
export type TodoFormProps = {
  id: string;
  class?: string;
  onSubmit: (form: FormController, values: TodoFormValues) => void | Promise<void>;
};

// cast types because initial values is an invalid state
const INITIAL_VALUES: Partial<TodoFormValues> = {
  title: "" as TodoTitle,
  description: "",
  todoAt: "" as ConventionalDate,
};

export function TodoForm(props: TodoFormProps): JSX.Element {
  const formStyles = classes("flex flex-col gap-4").add(props.class).build();
  const [form, { Form, Field: FormField }] = createForm<TodoFormValues>({
    initialValues: INITIAL_VALUES,
    validateOn: "blur",
  });

  const submit = (values: TodoFormValues): void => {
    const controller: FormController = { reset: () => reset(form) };
    props.onSubmit(controller, values);
  };

  return (
    <Form class={formStyles} id={props.id} action="#" onSubmit={submit}>
      <FormField name="title" validate={validateTitle}>
        {(field, props) => (
          <Field for="title" label="Título" error={field.error}>
            <Input
              {...props}
              required
              value={field.value}
              name="title"
              placeholder="Informe o título"
              status={getInputStatus(field.dirty, field.error)}
            />
          </Field>
        )}
      </FormField>

      <FormField name="description">
        {(field, props) => (
          <Field for="description" label="Descrição">
            <Input
              {...props}
              value={field.value}
              name="description"
              placeholder="Informe a descrição"
              status={getInputStatus(field.dirty, field.error)}
            />
          </Field>
        )}
      </FormField>

      <div class="flex gap-4 justify-between">
        <FormField name="status" validate={validateStatus}>
          {(field, props) => (
            <Field label="Status" for="status" error={field.error}>
              <Select
                {...props}
                required
                value={field.value}
                name="status"
                status={getInputStatus(field.dirty, field.error)}
              >
                <option hidden></option>
                <option value="todo">A fazer</option>
                <option value="in_progress">Em progresso</option>
                <option value="done">Feito</option>
              </Select>
            </Field>
          )}
        </FormField>

        <FormField name="todoAt" validate={validateTodoAt}>
          {(field, props) => (
            <Field for="todoAt" label="Data" error={field.error}>
              <Input
                {...props}
                value={field.value}
                name="todoAt"
                status={getInputStatus(field.dirty, field.error)}
                placeholder={`Ex: ${formatConventionalDate(new Date())}`}
              />
            </Field>
          )}
        </FormField>
      </div>
    </Form>
  );
}

function getInputStatus(dirty: boolean, error?: string): "ok" | "err" | undefined {
  if (error) {
    return "err";
  }
  return dirty ? "ok" : undefined;
}

async function validateTitle(value: unknown): Promise<string> {
  const title = parseTodoTitle(value);
  if (title.isOk()) {
    return "";
  }

  switch (title.value) {
    case "string":
      return "Informe o título";
    case "length":
      return "Título deve conter ao menos um caractere";
    default:
      return unreachable(title);
  }
}

async function validateStatus(value: unknown): Promise<string> {
  const status = parseTodoStatus(value);
  return status.isErr() ? "Selecione um dos status listados" : "";
}

async function validateTodoAt(value: unknown): Promise<string> {
  if (!value) {
    return "";
  } else if (typeof value === "string" && isConventionalDate(value)) {
    return "";
  }

  return "Data deve ser no formato DD/MM/YYYY";
}
