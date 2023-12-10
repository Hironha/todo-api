import { Switch, Match } from "solid-js";
import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../core/utils/classes";
import { unreachable } from "../../core/utils/unreachable";

export type TypographySize = "xs" | "sm" | "md" | "lg" | "xl" | "xxl";
export type TypographyWeight = "normal" | "light" | "bold";
export type TitleLevel = 1 | 2 | 3 | 4 | 5 | 6;

export type TypographyProps = JSX.HTMLAttributes<HTMLParagraphElement> & {
  /** @default "normal" */
  weight?: TypographyWeight;
  /** @default "md" */
  size?: TypographySize;
};

export function Typography(props: TypographyProps): JSX.Element {
  const styles = classes(getWeightStyle(props.weight ?? "normal"))
    .add(getSizeStyle(props.size ?? "md"))
    .add("mb-2")
    .add(props.class)
    .build();

  return <p class={styles}>{props.children}</p>;
}

export type TextProps = JSX.HTMLAttributes<HTMLSpanElement> & {
  /** @default "normal" */
  weight?: TypographyWeight;
  /** @default "md" */
  size?: TypographySize;
};

Typography.Text = (props: TextProps): JSX.Element => {
  const styles = classes(getWeightStyle(props.weight ?? "normal"))
    .add(getSizeStyle(props.size ?? "md"))
    .add(props.class)
    .build();

  return <span class={styles}>{props.children}</span>;
};

export type TitleProps = JSX.HTMLAttributes<HTMLHeadingElement> & {
  /** @default 1 */
  level?: TitleLevel;
  /** @default "bold" */
  weight?: TypographyWeight;
  /** @default "xl" */
  size?: TypographySize;
};

Typography.Title = (props: TitleProps): JSX.Element => {
  const styles = classes(getWeightStyle(props.weight ?? "bold"))
    .add(getSizeStyle(props.size ?? "xl"))
    .add("mb-2")
    .add(props.class)
    .build();

  const level = props.level ?? 1;

  return (
    <Switch fallback={<h1 class={styles}>{props.children}</h1>}>
      <Match when={level === 1}>
        <h1 class={styles}>{props.children}</h1>
      </Match>

      <Match when={level === 2}>
        <h2 class={styles}>{props.children}</h2>
      </Match>

      <Match when={level === 3}>
        <h3 class={styles}>{props.children}</h3>
      </Match>

      <Match when={level === 4}>
        <h4 class={styles}>{props.children}</h4>
      </Match>

      <Match when={level === 5}>
        <h5 class={styles}>{props.children}</h5>
      </Match>

      <Match when={level === 6}>
        <h6 class={styles}>{props.children}</h6>
      </Match>
    </Switch>
  );
};

function getWeightStyle(weight: TypographyWeight) {
  switch (weight) {
    case "light":
      return "font-light";
    case "normal":
      return "font-normal";
    case "bold":
      return "font-bold";
    default:
      return unreachable(weight);
  }
}

function getSizeStyle(size: TypographySize) {
  switch (size) {
    case "xs":
      return "text-xs";
    case "sm":
      return "text-sm";
    case "md":
      return "text-base";
    case "lg":
      return "text-xl";
    case "xl":
      return "text-2xl";
    case "xxl":
      return "text-3xl";
    default:
      return unreachable(size);
  }
}
