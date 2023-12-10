import { type JSX } from "solid-js/jsx-runtime";

import { Input } from "../ui/input";
import { type TodoStatus } from "../../core/entities/todo";

export type TodoFormValues = {
  title: string;
  description?: string;
  todoAt?: Date;
  status: TodoStatus;
};

export type TodoFormProps = {
  onSubmit: (values: TodoFormValues) => void;
};

export function TodoForm(props: TodoFormProps): JSX.Element {
  const submit = (event: Event): void => {
    event.preventDefault();
    // TODO: pass real values as parameter
    props.onSubmit({} as any);
  };

  return (
    <form class="flex flex-col gap-4" action="#" onSubmit={submit}>
      <Input name="title" label="Título" placeholder="Informe o título" status={{ kind: "ok" }} />
      <Input name="description" label="Descrição" placeholder="Informe a descrição" />
      <Input name="status" label="Status" placeholder="Informe o título" />
    </form>
  );
}
