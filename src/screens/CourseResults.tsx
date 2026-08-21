import { t } from "../i18n";
import { CourseResultsTable } from "../components/CourseResultsTable";
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
      <p class="text-[0.85em] opacity-75">{props.courseTitle}</p>
      <CourseResultsTable sections={props.sections} />

      <div class="flex justify-center">
        <button class="btn btn-outline btn-primary" disabled={props.busy} onClick={continueScreen}>
          {t().continueLabel}
        </button>
      </div>
    </>
  );
}
