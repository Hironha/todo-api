import { createSignal, onMount, type Accessor } from "solid-js";

const AVAILABLE_THEMES = ["light", "dark"] as const;

export type ThemeKind = (typeof AVAILABLE_THEMES)[number];

export type ThemeConfig = {
  current: Accessor<ThemeKind>;
  set: (kind: ThemeKind) => void;
};

export function useThemeConfig(): ThemeConfig {
  const [current, setCurrent] = createSignal<ThemeKind>("dark");
  let html: HTMLHtmlElement | null = null;

  const setTheme = (theme: ThemeKind): void => {
    html?.setAttribute("data-theme", theme);
    setCurrent(theme);
  };

  onMount(() => {
    html = document.querySelector("html");
    const initialTheme = html?.getAttribute("data-theme");
    if (isThemeKind(initialTheme)) {
      setCurrent(initialTheme);
    }
  });

  return { current, set: setTheme };
}

function isThemeKind(theme: unknown): theme is ThemeKind {
  return AVAILABLE_THEMES.includes(theme as any);
}
