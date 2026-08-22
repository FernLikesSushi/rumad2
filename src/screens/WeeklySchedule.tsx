import { For } from "solid-js";
import { t } from "../i18n";
import type { TuiScreenComponentProps } from "../types";

export function WeeklyScheduleScreen(props: TuiScreenComponentProps<"WeeklySchedule">) {
  function continueScreen() {
    props.send({ kind: "Continue" });
  }

  return (
    <>
      <h2>{t().weeklyScheduleTitle}</h2>
      <div class="overflow-x-auto max-w-full">
        <table class="table">
          <thead>
            <tr>
              <th>{t().weeklyScheduleColumns.period}</th>
              <For each={props.screen.days}>{(day) => <th>{day}</th>}</For>
            </tr>
          </thead>
          <tbody>
            <For each={props.screen.rows}>
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

      <button class="btn btn-outline btn-primary" disabled={props.busy} onClick={continueScreen}>
        {t().continueLabel}
      </button>
    </>
  );
}
