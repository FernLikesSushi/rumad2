import { t } from "../i18n";
import "./Spinner.css";

export function Spinner() {
  return <span class="spinner" role="status" aria-label={t().loading} />;
}
