import { t } from "../i18n";
export function Spinner() {
  return <span class="loading loading-spinner loading-lg" role="status" aria-label={t().loading} />;
}
