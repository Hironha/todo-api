import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../utils/classes";
import { unreachable } from "../../utils/unreachable";

export type TypographySize = "xs" | "sm" | "md" | "lg" | "xl" | "xxl";
export type TypographyWeight = "normal" | "light" | "bold";
export type TitleLevel = 1 | 2 | 3 | 4 | 5 | 6;

export type TypographyProps = {
  class?: string;
  /** @default "normal" */
  weight?: TypographyWeight;
  /** @default "md" */
  size?: TypographySize;
  children?: JSX.Element;
};

export function Typography(props: TypographyProps): JSX.Element {
  const styles = classes(getWeightStyle(props.weight ?? "normal"))
    .add(getSizeStyle(props.size ?? "md"))
    .add("mb-2")
    .add(props.class ?? "")
    .build();

  return <p class={styles}>{props.children}</p>;
}

export type TextProps = {
  class?: string;
  /** @default "normal" */
  weight?: TypographyWeight;
  /** @default "md" */
  size?: TypographySize;
  children?: JSX.Element;
};

Typography.Text = (props: TextProps): JSX.Element => {
  const styles = classes(getWeightStyle(props.weight ?? "normal"))
    .add(getSizeStyle(props.size ?? "md"))
    .add(props.class ?? "")
    .build();

  return <span class={styles}>{props.children}</span>;
};

export type TitleProps = {
  class?: string;
  /** @default 1 */
  level?: TitleLevel;
  /** @default "bold" */
  weight?: TypographyWeight;
  /** @default "xl" */
  size?: TypographySize;
  children?: JSX.Element;
};

Typography.Title = (props: TitleProps): JSX.Element => {
  const styles = classes(getWeightStyle(props.weight ?? "bold"))
    .add(getSizeStyle(props.size ?? "xl"))
    .add("mb-2")
    .add(props.class ?? "")
    .build();

  const level = props.level ?? 1;
  switch (level) {
    case 1:
      return <h1 class={styles}>{props.children}</h1>;
    case 2:
      return <h2 class={styles}>{props.children}</h2>;
    case 3:
      return <h3 class={styles}>{props.children}</h3>;
    case 4:
      return <h4 class={styles}>{props.children}</h4>;
    case 5:
      return <h5 class={styles}>{props.children}</h5>;
    case 6:
      return <h6 class={styles}>{props.children}</h6>;
    default:
      return unreachable(level);
  }
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
