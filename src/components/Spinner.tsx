import { t } from "../i18n";
import styles from "./Spinner.module.css";

export function Spinner() {
  return <span class={styles.spinner} role="status" aria-label={t().loading} />;
}
