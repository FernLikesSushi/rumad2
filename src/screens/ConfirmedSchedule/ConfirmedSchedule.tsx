import { t } from "../../i18n";
import { ConfirmedScheduleTable } from "./ConfirmedScheduleTable";
import { WeekCalendar } from "../../components/WeekCalendar";
import type { TuiScreenComponentProps } from "../api";

export function ConfirmedScheduleScreen(props: TuiScreenComponentProps<"ConfirmedSchedule">) {
  const events = () =>
    props.screen.courses.map((c) => ({
      label: `${c.course} ${c.section}`,
      meetings: c.meetings,
    }));

  return (
    <>
      <h2>{t().confirmedScheduleTitle}</h2>
      <ConfirmedScheduleTable courses={props.screen.courses} />
      <WeekCalendar events={events()} />
    </>
  );
}
