import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../utils/classes";

export type ContentProps = JSX.HTMLAttributes<HTMLDivElement>;

export function Content(props: ContentProps): JSX.Element {
  const styles = classes("p-4")
    .add(props.class ?? "")
    .build();

  return <div class={styles}>{props.children}</div>;
}
