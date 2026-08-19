import { createSignal, Show } from "solid-js";
import type { ClassifiedScreen } from "./types";
import { Header } from "./components/Header";
import { Connect } from "./Connect";
import { TuiRouter } from "./TuiRouter";
import "./App.css";

// The thin top-level shell: picks between the two pages based on whether
// a connection exists yet. `Connect` owns the initial `connect` call and
// hands off the resulting screen here via `setConnected`; from that point
// on `TuiRouter` owns everything TUI-state-related (its own resource,
// dialogs, busy state) until the session ends and calls `onDisconnected`,
// which resets `connected` back to null and remounts `Connect`.
function App() {
  const [connected, setConnected] = createSignal<ClassifiedScreen | null>(null);

  return (
    <main class="container">
      <Header />

      <Show when={connected()} fallback={<Connect onConnected={setConnected} />}>
        {(initial) => <TuiRouter initial={initial()} onDisconnected={() => setConnected(null)} />}
      </Show>
    </main>
  );
}

export default App;
