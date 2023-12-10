import { type JSX } from "solid-js/jsx-runtime";
import { Typography } from "./typography";
import { classes } from "../../core/utils/classes";

export type EmptyProps = {
  class?: string;
  message: string;
};

export function Empty(props: EmptyProps): JSX.Element {
  const styles = classes("w-ful flex justify-center items-center")
    .add(props.class ?? "")
    .build();

  return (
    <div class={styles}>
      <Typography>{props.message}</Typography>
    </div>
  );
}
