import { For } from "solid-js";
import { t } from "../i18n";
import type { ScheduleRow, Send } from "../types";

export function WeeklyScheduleScreen(props: { days: string[]; rows: ScheduleRow[]; busy: boolean; send: Send }) {
  // Read-only grid: nothing to select, just "Enter to continue" -- no
  // confirmed exit keystroke to offer a button for (see `RumadScreen for
  // WeeklyScheduleScreen` in the backend).
  function continueScreen() {
    props.send({ kind: "Line", text: "" });
  }

  return (
    <>
      <h2>{t().weeklyScheduleTitle}</h2>
      <div class="table-scroll">
        <table class="courses">
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

      <button disabled={props.busy} onClick={continueScreen}>
        {t().continueLabel}
      </button>
    </>
  );
}
