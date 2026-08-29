import { createMemo, createSignal } from "solid-js";
import { CourseSection, Meeting } from "./tui";
import { t } from "../i18n";
import { showToast } from "./toast";

export interface Course {
  courseCode: string;
  section: string;
  room: string; // Some courses may not have a room assigned yet
  schedule: string;
  credits: number;
  professor: string | undefined; // Some courses may not have a professor assigned yet
  meetings: Meeting[];
}

export interface ClassProfile {
  courses: Course[];
}

export interface ClassEditorState {
  profiles: Record<string, ClassProfile>;
  selectedProfileName: string | null;
}

export interface ValidateScheduleResult {
  isValid: boolean;
  overlappingCourses: [CourseSection, CourseSection][];
}

const STORAGE_KEY = "rumad-class-profiles";
const emptyState: ClassEditorState = {
  profiles: {},
  selectedProfileName: null,
};

// `STORAGE_KEY`/`emptyState` above must come first: this is called
// immediately below to seed the signal's initial value, and a `const`
// referenced before its own declaration line has run throws
// ("Cannot access before initialization"), not just reads as undefined.
function loadInitial(): ClassEditorState {
  if (typeof localStorage === "undefined") {
    return emptyState;
  }
  try {
    return (
      JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "null") ?? emptyState
    );
  } catch {
    return emptyState;
  }
}

function persist(value: ClassEditorState) {
  setStoredState(value);
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
  }
}

const [storedState, setStoredState] =
  createSignal<ClassEditorState>(loadInitial());

export const classEditorState = createMemo(() => storedState());
export const selectedProfile = createMemo<ClassProfile | null>(() => {
  const { profiles, selectedProfileName } = storedState();
  if (selectedProfileName === null) {
    return null;
  }
  return profiles[selectedProfileName] ?? null;
});

export function saveClassProfile(name: string, profile: ClassProfile) {
  const current = storedState();
  persist({ ...current, profiles: { ...current.profiles, [name]: profile } });
}

// Shared by `addCourseToProfile`/`removeCourseFromProfile` below --
// `updater` returning the same `courses` reference back (no actual
// change, e.g. adding a course that's already there) skips both the
// `saveClassProfile` write and the toast, so a no-op click doesn't pop one.
function updateProfileCourses(name: string | null, updater: (courses: Course[]) => Course[]): boolean {
  if (!name) return false;
  const profile = storedState().profiles[name];
  if (!profile) return false;
  const courses = updater(profile.courses);
  if (courses === profile.courses) return false;
  saveClassProfile(name, { courses });
  return true;
}

export function addCourseToProfile(name: string | null, course: Course) {
  const added = updateProfileCourses(name, (courses) => {
    const alreadyAdded = courses.some((c) => c.courseCode === course.courseCode && c.section === course.section);
    return alreadyAdded ? courses : [...courses, course];
  });
  if (added) showToast(t().courseAddedToast);
}

export function removeCourseFromProfile(name: string | null, courseCode: string, section: string) {
  const removed = updateProfileCourses(name, (courses) => {
    const filtered = courses.filter((c) => !(c.courseCode === courseCode && c.section === section));
    return filtered.length === courses.length ? courses : filtered;
  });
  if (removed) showToast(t().courseRemovedToast);
}

export function deleteClassProfile(name: string) {
  const current = storedState();
  const { [name]: _removed, ...rest } = current.profiles;
  persist({
    profiles: rest,
    selectedProfileName:
      current.selectedProfileName === name ? null : current.selectedProfileName,
  });
}

export function selectClassProfile(name: string | null) {
  persist({ ...storedState(), selectedProfileName: name });
}

/// Ensure no class has meetings that overlap with another class's meetings. Returns `true` if the schedule is valid, `false` if any two classes overlap.
function validateSchedule(courses: CourseSection[]): ValidateScheduleResult {
  const overlappingCourses: [CourseSection, CourseSection][] = [];

  for (let i = 0; i < courses.length; i++) {
    for (let j = i + 1; j < courses.length; j++) {
      const courseA = courses[i];
      const courseB = courses[j];

      if (doCoursesOverlap(courseA, courseB)) {
        overlappingCourses.push([courseA, courseB]);
      }
    }
  }

  return {
    isValid: overlappingCourses.length === 0,
    overlappingCourses,
  };
}

function doCoursesOverlap(
  courseA: CourseSection,
  courseB: CourseSection,
): boolean {
  if (courseA.section === courseB.section) {
    return false; // Same section, no overlap
  }
  if (courseA.meetings.length === 0 || courseB.meetings.length === 0) {
    return false; // No meetings, no overlap
  }

  for (const meetingA of courseA.meetings) {
    for (const meetingB of courseB.meetings) {
      if (
        meetingA.day === meetingB.day &&
        meetingA.startMinutes < meetingB.endMinutes &&
        meetingB.startMinutes < meetingA.endMinutes
      ) {
        return true; // Found an overlap
      }
    }
  }

  return false; // No overlaps found
}
