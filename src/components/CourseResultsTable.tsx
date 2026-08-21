import { For } from "solid-js";
import { t } from "../i18n";
import type { CourseSection } from "../types";

export function CourseResultsTable(props: { sections: CourseSection[] }) {
  return (
    <div class="overflow-x-auto max-w-full">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>{t().courseResultsColumns.section}</th>
            <th>{t().courseResultsColumns.room}</th>
            <th>{t().courseResultsColumns.schedule}</th>
            <th>{t().courseResultsColumns.credits}</th>
            <th>{t().courseResultsColumns.professor}</th>
            <th>{t().courseResultsColumns.capacity}</th>
            <th>{t().courseResultsColumns.used}</th>
            <th>{t().courseResultsColumns.available}</th>
          </tr>
        </thead>
        <tbody>
          <For each={props.sections}>
            {(s) => (
              <tr>
                <td>{s.section}</td>
                <td>{s.room}</td>
                <td>{s.schedule}</td>
                <td>{s.credits}</td>
                <td>{s.professor}</td>
                <td>{s.capacity}</td>
                <td>{s.used}</td>
                <td>{s.available}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
}
