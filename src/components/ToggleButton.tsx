export function ToggleButton(props: { label: string; active: boolean; onClick: () => void }) {
  return (
    <button class="btn btn-lg" classList={{ "btn-active": props.active }} onClick={props.onClick}>
      {props.label}
    </button>
  );
}
