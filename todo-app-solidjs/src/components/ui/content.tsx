import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../core/utils/classes";

export type ContentProps = JSX.HTMLAttributes<HTMLDivElement>;

export function Content(props: ContentProps): JSX.Element {
  const styles = (): string => {
    return classes("p-6").add(props.class).build();
  };
  return <div class={styles()}>{props.children}</div>;
}
