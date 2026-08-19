import { For } from "solid-js";
import { t } from "../i18n";
import type { ScheduleRow } from "../types";

export function WeeklyScheduleScreen(props: {
  days: string[];
  rows: ScheduleRow[];
  busy: boolean;
  onContinue: () => void;
}) {
  return (
    <>
      <h2>{t().weeklyScheduleTitle}</h2>
      <div class="table-scroll">
        <table class="courses">
          <thead>
            <tr>
              <th>Periodos</th>
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

      <button disabled={props.busy} onClick={props.onContinue}>
        {t().continueLabel}
      </button>
    </>
  );
}
