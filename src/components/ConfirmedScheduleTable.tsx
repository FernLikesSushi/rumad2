import { For } from "solid-js";
import { t } from "../i18n";
import type { ConfirmedCourse } from "../types";

export function ConfirmedScheduleTable(props: { courses: ConfirmedCourse[] }) {
  return (
    <div class="overflow-x-auto max-w-full">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>{t().confirmedScheduleColumns.course}</th>
            <th>{t().confirmedScheduleColumns.section}</th>
            <th>{t().confirmedScheduleColumns.credits}</th>
            <th>{t().confirmedScheduleColumns.room}</th>
            <th>{t().confirmedScheduleColumns.professor}</th>
          </tr>
        </thead>
        <tbody>
          <For each={props.courses}>
            {(c) => (
              <tr>
                <td>{c.course}</td>
                <td>{c.section}</td>
                <td>{c.credits}</td>
                <td>{c.room}</td>
                <td>{c.professor}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
}
