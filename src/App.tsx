import { Footer } from './components/Footer';
import { HashRouter, Route } from "@solidjs/router";
import { Heart } from "lucide-solid";
import { Connect } from "./page/Connect";
import { TuiRouter } from "./page/TuiRouter";
import { t } from "./i18n";
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
        // `justify-start`, not `-center`: this wraps every page's children as
        // one group, so centering it would re-center (and visibly shift) the
        // header whenever the rest of the content is short or still loading
        // (e.g. TuiRouter before its first `get_screen` resolves, when Header
        // is briefly the only child). Each page centers its own content area
        // independently instead (see Connect's button wrapper's own `flex-1`).
        <main class="flex flex-col justify-start text-center px-4 py-4 min-h-screen">
          {props.children}
          <Footer />
        </main>
      )}
    >
      <Route path="/" component={Connect} />
      <Route path="/session" component={TuiRouter} />
    </HashRouter>
  );
}

export default App;
