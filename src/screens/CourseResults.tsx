import { For } from "solid-js";
import { t } from "../i18n";
import type { CourseSection, Send } from "../types";

export function CourseResultsScreen(props: {
  courseCode: string;
  courseTitle: string;
  sections: CourseSection[];
  busy: boolean;
  send: Send;
}) {
  // Read-only screen: nothing to select, just "Enter to continue" (a bare
  // Line). PF4-to-leave is handled by App.tsx's shared exit control.
  function continueScreen() {
    props.send({ kind: "Line", text: "" });
  }

  return (
    <>
      <h2>{props.courseCode}</h2>
      <p class="hint">{props.courseTitle}</p>
      <div class="table-scroll">
        <table class="courses">
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

      <div class="row">
        <button disabled={props.busy} onClick={continueScreen}>
          {t().continueLabel}
        </button>
      </div>
    </>
  );
}
