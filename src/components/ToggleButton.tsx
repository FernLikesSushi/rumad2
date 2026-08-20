import styles from "./ToggleButton.module.css";

export function ToggleButton(props: { label: string; active: boolean; onClick: () => void }) {
  return (
    <button class={styles["toggle-button"]} classList={{ [styles.active]: props.active }} onClick={props.onClick}>
      {props.label}
    </button>
  );
}
