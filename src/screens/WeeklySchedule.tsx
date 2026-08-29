import { t } from "../i18n";
import { ScheduleTable } from "../components/ScheduleTable";
import type { TuiScreenComponentProps } from "./api";

export function WeeklyScheduleScreen(props: TuiScreenComponentProps<"WeeklySchedule">) {
  return (
    <>
      <h2>{t().weeklyScheduleTitle}</h2>
      <ScheduleTable days={props.screen.days} rows={props.screen.rows} />
    </>
  );
}
