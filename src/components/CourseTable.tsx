import { For } from "solid-js";
import { t } from "../i18n";
import type { ScheduleCourse } from "../data/tui";
import { CourseLink } from "./CourseLink";

export function CourseTable(props: { courses: ScheduleCourse[] }) {
  return (
    <table class="table table-zebra">
      <thead>
        <tr>
          <th>{t().matriculaColumns.course}</th>
          <th>{t().matriculaColumns.section}</th>
          <th>{t().matriculaColumns.credits}</th>
          <th>{t().matriculaColumns.status}</th>
        </tr>
      </thead>
      <tbody>
        <For each={props.courses}>
          {(c) => (
            <tr>
              <td><CourseLink course={c.course} /></td>
              <td>{c.section}</td>
              <td>{c.credits}</td>
              <td>{c.status}</td>
            </tr>
          )}
        </For>
      </tbody>
    </table>
  );
}
