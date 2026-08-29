import { t } from "../../i18n";
import { WeeklyScheduleTable } from "./WeeklyScheduleTable";
import type { TuiScreenComponentProps } from "../api";

/** Read-only weekly calendar grid of the student's confirmed schedule. */
export function WeeklyScheduleScreen(props: TuiScreenComponentProps<"WeeklySchedule">) {
  return (
    <>
      <h2>{t().weeklyScheduleTitle}</h2>
      <WeeklyScheduleTable days={props.screen.days} rows={props.screen.rows} />
    </>
  );
}
