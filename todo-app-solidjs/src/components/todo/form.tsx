import { type JSX } from "solid-js/jsx-runtime";

import { Input, Field, Select } from "../ui/form";
import { type TodoStatus } from "../../core/entities/todo";
import { DateUtils } from "../../core/utils/date";
import { classes } from "../../core/utils/classes";

export type TodoFormValues = {
  title: string;
  description?: string;
  todoAt?: Date;
  status: TodoStatus;
};

export type TodoFormProps = {
  id: string;
  class?: string;
  onSubmit: (values: TodoFormValues) => void;
};

export function TodoForm(props: TodoFormProps): JSX.Element {
  const formStyles = classes("flex flex-col gap-4").add(props.class).build();

  const submit = (event: Event): void => {
    event.preventDefault();
    // TODO: pass real values as parameter
    props.onSubmit({} as any);
  };

  return (
    <form class={formStyles} id={props.id} action="#" onSubmit={submit}>
      <Field for="title" label="Título">
        <Input name="title" placeholder="Informe o título" />
      </Field>

      <Field for="description" label="Descrição">
        <Input name="description" placeholder="Informe a descrição" />
      </Field>

      <div class="flex gap-4 justify-between">
        <Field label="Status" for="status">
          <Select name="status" value="todo">
            <option value="todo">A fazer</option>
            <option value="in_progress">Em progresso</option>
            <option value="done">Feito</option>
          </Select>
        </Field>

        <Field for="todoAt" label="Data">
          <Input
            name="status"
            placeholder={`Ex: ${DateUtils.toLocalYmd(new Date())}`}
          />
        </Field>
      </div>
    </form>
  );
}
