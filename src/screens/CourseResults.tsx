import { createMemo } from "solid-js";
import { CourseResultsTable } from "../components/CourseResultsTable";
import { WeekCalendar } from "../components/WeekCalendar";
import type { TuiScreenComponentProps } from "../types";

export function CourseResultsScreen(props: TuiScreenComponentProps<"CourseResults">) {
  const events = createMemo(() =>
    props.screen.sections.map((s) => ({
      label: `${s.section} - ${s.professor}`,
      meetings: s.meetings,
    })));

  return (
    <>
      <h2>{props.screen.courseCode}</h2>
      <p class="text-[0.85em] opacity-75">{props.screen.courseTitle}</p>
      <CourseResultsTable sections={props.screen.sections} />
      <WeekCalendar events={events()} />
    </>
  );
}
