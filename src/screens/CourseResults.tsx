import { createMemo } from "solid-js";
import { CourseResultsTable } from "../components/CourseResultsTable";
import { WeekCalendar } from "../components/WeekCalendar";
import { CourseLink } from "../components/CourseLink";
import type { TuiScreenComponentProps } from "./api";

export function CourseResultsScreen(
  props: TuiScreenComponentProps<"CourseResults">,
) {
  const events = createMemo(() =>
    props.screen.sections.map((s) => ({
      label: `${s.section} - ${s.professor}`,
      meetings: s.meetings,
    })),
  );

  return (
    <>
      <h2>
        <CourseLink course={props.screen.courseCode} /> -{" "}
        {props.screen.courseTitle}
      </h2>

      <CourseResultsTable courseCode={props.screen.courseCode} sections={props.screen.sections} />
      <WeekCalendar events={events()} />
    </>
  );
}
