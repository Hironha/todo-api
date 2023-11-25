import { createSignal } from "solid-js";

import { Typography } from "./components/ui/typography";

function App() {
  const [count, setCount] = createSignal(0);

  const increment = () => setCount((count) => count + 1);

  return (
    <div class="flex flex-col gap-2 items-start">
      <h1 class="text-sm">Vite + Solid</h1>

      <button class="btn btn-primary btn-sm" onClick={increment}>
        count is {count()}
      </button>

      <Typography weight="bold" size="md">
        hello world
      </Typography>

      <p>
        Edit <code>src/App.tsx</code> and save to test HMR
      </p>
    </div>
  );
}

export default App;
