import { For } from "solid-js";
import { t } from "../i18n";
import type { ScheduleRow } from "../types";

export function ScheduleTable(props: { days: string[]; rows: ScheduleRow[] }) {
  return (
    <div class="overflow-x-auto max-w-full">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>{t().weeklyScheduleColumns.period}</th>
            <For each={props.days}>{(day) => <th>{day}</th>}</For>
          </tr>
        </thead>
        <tbody>
          <For each={props.rows}>
            {(row) => (
              <tr>
                <td>{row.period}</td>
                <For each={row.days}>{(cell) => <td>{cell}</td>}</For>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
}
