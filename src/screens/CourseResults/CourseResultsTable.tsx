import { For } from "solid-js";
import { t } from "../../i18n";
import type { CourseSection } from "../../data/tui";
import { RoomLink } from "../../components/RoomLink";
import { addCourseToProfile, classEditorState, selectedProfile } from "../../data/classEditor";
import { Plus } from "lucide-solid";

// `courseCode` is a prop, not part of `CourseSection` -- needed to build
// the `Course` the "add" button saves into the selected `ClassProfile`.
export function CourseResultsTable(props: {
  courseCode: string;
  sections: CourseSection[];
}) {
  function addToProfile(section: CourseSection) {
    addCourseToProfile(classEditorState().selectedProfileName, {
      courseCode: props.courseCode,
      section: section.section,
      room: section.room,
      schedule: section.schedule,
      credits: section.credits,
      professor: section.professor,
      meetings: section.meetings,
    });
  }

  return (
    <div class="overflow-x-auto max-w-full">
      <table class="table table-zebra">
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
            <th />
          </tr>
        </thead>
        <tbody>
          <For each={props.sections}>
            {(s) => (
              <tr>
                <td>{s.section}</td>
                <td>
                  <RoomLink room={s.room} />
                </td>
                <td>{s.schedule}</td>
                <td>{s.credits}</td>
                <td>{s.professor}</td>
                <td>{s.capacity}</td>
                <td>{s.used}</td>
                <td>{s.available}</td>
                <td>
                  <div
                    class="tooltip"
                    // disable tooltip if selectedProfile is valid
                    data-tip={selectedProfile() ? undefined : t().addToClassProfileDisabledHint}
                  >
                    <button
                      class="btn btn-square btn-sm"
                      aria-label={t().addToClassProfile}
                      disabled={!selectedProfile()}
                      onClick={() => addToProfile(s)}
                    >
                      <Plus class="size-4" />
                    </button>
                  </div>
                </td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
}
