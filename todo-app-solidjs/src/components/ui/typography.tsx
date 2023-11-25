import { type JSX } from "solid-js/jsx-runtime";

import { classes } from "../../utils/classes";
import { unreachable } from "../../utils/unreachable";

export type TypographySize = "xs" | "sm" | "md" | "lg" | "xl" | "xxl";
export type TypographyWeight = "normal" | "light" | "bold";

export type TypographyProps = {
  class?: string;
  /** @default "normal" */
  weight?: "normal" | "light" | "bold";
  /** @default "md" */
  size?: TypographySize;
  children?: JSX.Element;
};

export function Typography(props: TypographyProps): JSX.Element {
  const styles = classes(getWeightStyle(props.weight ?? "normal"))
    .add(getSizeStyle(props.size ?? "md"))
    .add(props.class ?? "")
    .build();

  return <p class={styles}>{props.children}</p>;
}

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
      return "text-4xl";
    default:
      return unreachable(size);
  }
}
