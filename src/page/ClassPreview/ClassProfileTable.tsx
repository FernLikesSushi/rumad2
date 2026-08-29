import { For } from "solid-js";
import { t } from "../../i18n";
import type { Course } from "../../data/classEditor";
import { CourseLink } from "../../components/CourseLink";
import { RoomLink } from "../../components/RoomLink";
import { Trash2 } from "lucide-solid";

export function ClassProfileTable(props: { courses: Course[]; onRemove: (course: Course) => void }) {
  return (
    <div class="overflow-x-auto max-w-full">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>{t().classProfileColumns.course}</th>
            <th>{t().classProfileColumns.section}</th>
            <th>{t().classProfileColumns.room}</th>
            <th>{t().classProfileColumns.schedule}</th>
            <th>{t().classProfileColumns.credits}</th>
            <th>{t().classProfileColumns.professor}</th>
            <th />
          </tr>
        </thead>
        <tbody>
          <For each={props.courses}>
            {(c) => (
              <tr>
                <td>
                  <CourseLink course={c.courseCode} />
                </td>
                <td>{c.section}</td>
                <td>
                  <RoomLink room={c.room} />
                </td>
                <td>{c.schedule}</td>
                <td>{c.credits}</td>
                <td>{c.professor}</td>
                <td>
                  <button
                    class="btn btn-square btn-sm btn-ghost text-error"
                    aria-label={t().removeFromClassProfile}
                    onClick={() => props.onRemove(c)}
                  >
                    <Trash2 class="size-4" />
                  </button>
                </td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
}
