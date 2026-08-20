import { HashRouter, Route } from "@solidjs/router";
import { Connect } from "./page/Connect";
import { TuiRouter } from "./page/TuiRouter";
import "./App.css";

// The thin top-level shell: a two-route router picking between the two
// pages. `Connect` navigates to "/session" after a successful `connect`;
// `TuiRouter` fetches its own starting screen on mount rather than being
// handed one, and navigates back to "/" once the session ends (explicit
// logout, or the remote's own "PROCESO CONCLUIDO").
function App() {
  return (
    <HashRouter
      root={(props) => (
        <main class="flex flex-col justify-center text-center px-4 py-4">
          {props.children}
        </main>
      )}
    >
      <Route path="/" component={Connect} />
      <Route path="/session" component={TuiRouter} />
    </HashRouter>
  );
}

export default App;
