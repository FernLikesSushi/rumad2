export function Toggle(props: { label: string; checked: boolean; onChange: (checked: boolean) => void }) {
  return (
    <label class="flex items-center gap-2 text-xs opacity-60 cursor-pointer">
      <input
        type="checkbox"
        class="toggle toggle-primary"
        checked={props.checked}
        onChange={(e) => props.onChange(e.currentTarget.checked)}
      />
      {props.label}
    </label>
  );
}
