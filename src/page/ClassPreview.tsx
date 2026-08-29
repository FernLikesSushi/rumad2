import { createSignal, For, Match, Show, Switch } from "solid-js";
import { Header } from "../components/Header";
import { NewProfileDialog } from "../components/NewProfileDialog";
import { ClassProfileTable } from "../components/ClassProfileTable";
import { WeekCalendar } from "../components/WeekCalendar";
import {
  classEditorState,
  removeCourseFromProfile,
  saveClassProfile,
  selectClassProfile,
  selectedProfile,
  type Course,
} from "../data/classEditor";
import { t } from "../i18n";
import { Calendar, Plus, Table } from "lucide-solid";

type View = "table" | "calendar";

export function ClassPreview() {
  const [isCreating, setIsCreating] = createSignal(false);
  const [view, setView] = createSignal<View>("table");

  function createProfile(name: string) {
    saveClassProfile(name, { courses: [] });
    selectClassProfile(name);
    setIsCreating(false);
  }

  function removeCourse(course: Course) {
    removeCourseFromProfile(classEditorState().selectedProfileName, course.courseCode, course.section);
  }

  return (
    <>
      <Header />
      <div class="flex flex-col items-center justify-center gap-4">
        {/* Profile selector */}
        <div class="flex flex-row items-center gap-4 w-64">
          <select
            class="select"
            value={classEditorState().selectedProfileName ?? ""}
            onChange={(e) => selectClassProfile(e.currentTarget.value || null)}
          >
            <option value="" disabled={true}>
              {t().classPreviewPicker}
            </option>
            <For each={Object.keys(classEditorState().profiles)}>
              {(name) => <option value={name}>{name}</option>}
            </For>
          </select>
          <button class="btn btn-square" onClick={() => setIsCreating(true)}>
            <Plus />
          </button>
        </div>

        <Show when={selectedProfile()}>
          {(profile) => (
            <>
              {/* Segmented control */}
              <div class="join">
                <button
                  class="btn join-item"
                  classList={{ "btn-active": view() === "table" }}
                  onClick={() => setView("table")}
                >
                  <Table class="size-4" />
                  {t().classPreviewView.table}
                </button>
                <button
                  class="btn join-item"
                  classList={{ "btn-active": view() === "calendar" }}
                  onClick={() => setView("calendar")}
                >
                  <Calendar class="size-4" />
                  {t().classPreviewView.calendar}
                </button>
              </div>

              {/* Table or calendar */}
              <div class="w-full min-w-3xs px-12">
                <Switch>
                  <Match when={view() === "table"}>
                    <ClassProfileTable courses={profile().courses} onRemove={removeCourse} />
                  </Match>
                  <Match when={view() === "calendar"}>
                    <WeekCalendar
                      events={profile().courses.map((c) => ({
                        label: `${c.courseCode} ${c.section}`,
                        meetings: c.meetings,
                        onRemove: () => removeCourse(c),
                      }))}
                    />
                  </Match>
                </Switch>
              </div>
            </>
          )}
        </Show>

        <Show when={!selectedProfile()}>
          <p>{t().classPreviewEmpty}</p>
        </Show>
      </div>
      <NewProfileDialog
        open={isCreating()}
        existingNames={Object.keys(classEditorState().profiles)}
        onCreate={createProfile}
        onClose={() => setIsCreating(false)}
      />
    </>
  );
}
