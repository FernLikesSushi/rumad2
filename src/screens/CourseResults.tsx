import { CourseResultsTable } from "../components/CourseResultsTable";
import type { TuiScreenComponentProps } from "../types";

export function CourseResultsScreen(props: TuiScreenComponentProps<"CourseResults">) {
  return (
    <>
      <h2>{props.screen.courseCode}</h2>
      <p class="text-[0.85em] opacity-75">{props.screen.courseTitle}</p>
      <CourseResultsTable sections={props.screen.sections} />
    </>
  );
}
