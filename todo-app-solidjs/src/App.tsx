import { createSignal } from "solid-js";

import { Typography } from "./components/ui/typography";
import { useThemeConfig } from "./utils/theme.";

function App() {
  const [count, setCount] = createSignal(0);
  const themeConfig = useThemeConfig();

  const increment = (): void => {
    setCount((count) => count + 1);
  };

  const setLightTheme = (): void => themeConfig.set("light");

  const setDarkTheme = (): void => themeConfig.set("dark");

  return (
    <div class="flex flex-col gap-2 items-start">
      <Typography.Title level={1}>Vite + Solid</Typography.Title>

      <button class="btn btn-primary btn-sm" onClick={increment}>
        count is {count()}
      </button>

      <button class="btn btn-primary btn-sm" onClick={setLightTheme}>
        set light theme
      </button>

      <button class="btn btn-primary btn-sm" onClick={setDarkTheme}>
        set dark theme
      </button>

      <Typography>
        Edit <code>src/App.tsx</code> and save to test HMR
      </Typography>
    </div>
  );
}

export default App;
