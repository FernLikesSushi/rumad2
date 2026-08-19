import { For } from "solid-js";
import { t } from "../i18n";
import type { CourseSection } from "../types";

export function CourseResultsScreen(props: {
  courseCode: string;
  courseTitle: string;
  sections: CourseSection[];
  busy: boolean;
  onContinue: () => void;
  onExit: () => void;
}) {
  return (
    <>
      <h2>{props.courseCode}</h2>
      <p class="hint">{props.courseTitle}</p>
      <div class="table-scroll">
        <table class="courses">
          <thead>
            <tr>
              <th>Sec.</th>
              <th>Salon</th>
              <th>Periodos</th>
              <th>Crd.</th>
              <th>Profesor</th>
              <th>Cap.</th>
              <th>Uti.</th>
              <th>Disp.</th>
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
        <button disabled={props.busy} onClick={props.onContinue}>
          {t().continueLabel}
        </button>
        <button disabled={props.busy} onClick={props.onExit}>
          {t().screenExit}
        </button>
      </div>
    </>
  );
}
