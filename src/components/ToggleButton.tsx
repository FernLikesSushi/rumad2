import "./ToggleButton.css";

export function ToggleButton(props: { label: string; active: boolean; onClick: () => void }) {
  return (
    <button class="toggle-button" classList={{ active: props.active }} onClick={props.onClick}>
      {props.label}
    </button>
  );
}
