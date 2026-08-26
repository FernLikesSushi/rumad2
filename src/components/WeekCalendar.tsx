import { createMemo, For, Show } from "solid-js";
import { t } from "../i18n";
import type { Meeting } from "../types";
import { X } from "lucide-solid";

// What `WeekCalendar` actually plots -- decoupled from `CourseSection`/
// `ConfirmedCourse` so it can render either (or anything else with
// meetings) without depending on one specific screen's data shape.
// `onRemove` is optional: only `ClassPreview`'s calendar view sets it, so
// every meeting box gets a remove control only there, not on the
// read-only `ConfirmedSchedule`/`CourseResults` calendars.
export type CalendarEvent = { label: string; meetings: Meeting[]; onRemove?: () => void };


const PX_PER_HOUR = 48;
const EVENT_COLORS = [
  "bg-primary/15 border-primary text-primary",
  "bg-secondary/15 border-secondary text-secondary",
  "bg-accent/15 border-accent text-accent",
  "bg-info/15 border-info text-info",
  "bg-success/15 border-success text-success",
  "bg-warning/15 border-warning text-warning",
];

type Bounds = { start: number; end: number };

/**
 * Formats a time in minutes since midnight into a 12-hour clock string with "am"/"pm" suffix.
 * @param minutes Minutes since midnight (0-1439).
 * @returns A string representing the time in 12-hour format, e.g., "1:30pm".
 */
function formatTime(minutes: number): string {
  const h24 = Math.floor(minutes / 60);
  const m = minutes % 60;
  const period = h24 >= 12 ? "pm" : "am";
  const h12 = h24 % 12 === 0 ? 12 : h24 % 12;
  return m === 0 ? `${h12}${period}` : `${h12}:${m.toString().padStart(2, "0")}${period}`;
}

function heightFor(minutes: number): number {
  return (minutes / 60) * PX_PER_HOUR;
}

// Left-hand column of hour labels.
function TimeGutter(props: { hours: number[]; bounds: Bounds }) {
  return (
    <div class="w-12 shrink-0">
      <div class="h-6" />
      <div class="relative" style={{ height: `${heightFor(props.bounds.end - props.bounds.start)}px` }}>
        <For each={props.hours}>
          {(h) => (
            <div
              class="absolute right-1 -translate-y-1/2 text-[0.65rem] opacity-60"
              style={{ top: `${heightFor(h - props.bounds.start)}px` }}
            >
              {formatTime(h)}
            </div>
          )}
        </For>
      </div>
    </div>
  );
}

// One positioned, colored box for a single event meeting.
function MeetingBox(props: { label: string; meeting: Meeting; bounds: Bounds; color: string; onRemove?: () => void }) {
  return (
    <div
      class={`absolute left-0.5 right-0.5 rounded-md border px-1 py-0.5 text-[0.65rem] leading-tight overflow-hidden ${props.color}`}
      style={{
        top: `${heightFor(props.meeting.startMinutes - props.bounds.start)}px`,
        height: `${heightFor(props.meeting.endMinutes - props.meeting.startMinutes)}px`,
      }}
    >
      {/* Remove button */}
      <Show when={props.onRemove}>
        {(onRemove) => (
          <button
            class="absolute top-0.5 right-0.5 opacity-70 hover:opacity-100"
            aria-label={t().removeFromClassProfile}
            onClick={onRemove()}
          >
            <X class="size-3" />
          </button>
        )}
      </Show>
      <div class="font-semibold truncate text-md pr-3">{props.label}</div>
      <div class="truncate text-md">
        {formatTime(props.meeting.startMinutes)}-{formatTime(props.meeting.endMinutes)}
      </div>
    </div>
  );
}

// One day's column: header, hour gridlines, and that day's meeting boxes.
function DayColumn(props: { day: number; hours: number[]; bounds: Bounds; events: CalendarEvent[] }) {
  return (
    <div class="flex-1 min-w-24 border-l border-base-300">
      <div class="h-6 text-xs text-center opacity-60">{t().weekdaysShort[props.day - 1]}</div>
      <div class="relative" style={{ height: `${heightFor(props.bounds.end - props.bounds.start)}px` }}>
        <For each={props.hours}>
          {(h) => (
            <div
              class="absolute left-0 right-0 border-t border-base-300"
              style={{ top: `${heightFor(h - props.bounds.start)}px` }}
            />
          )}
        </For>
        <For each={props.events}>
          {(event, i) => (
            <For each={event.meetings.filter((m) => m.day === props.day)}>
              {(meeting) => (
                <MeetingBox
                  label={event.label}
                  meeting={meeting}
                  bounds={props.bounds}
                  // cycle through a small palette of colors for each event,
                  // so overlapping/conflicting events are visually distinct
                  color={EVENT_COLORS[i() % EVENT_COLORS.length]}
                  onRemove={event.onRemove}
                />
              )}
            </For>
          )}
        </For>
      </div>
    </div>
  );
}

const ALL_DAYS = [1, 2, 3, 4, 5, 6];

// Tuesdays and Thursdays 10:30am-12:00pm are free -- a fixed institutional
// block shown on every calendar, not part of any screen's own data.
const UNIVERSAL_PERIOD: CalendarEvent = {
  label: "Hora Universal",
  meetings: [
    { day: 2, startMinutes: 10 * 60 + 30, endMinutes: 12 * 60 },
    { day: 4, startMinutes: 10 * 60 + 30, endMinutes: 12 * 60 },
  ],
};

// Plots a list of labeled events on a Mon-Sat time grid, one color per
// event, so overlapping/conflicting schedules are visible at a glance --
// built from each event's already Rust-parsed `meetings`, not re-derived
// from any raw schedule string.
export function WeekCalendar(props: { events: CalendarEvent[] }) {
  const events = createMemo(() => [UNIVERSAL_PERIOD, ...props.events]);

  const bounds = createMemo<Bounds>(() => {
    const all = events().flatMap((e) => e.meetings);
    // 7am-7pm covers the typical class day -- widened only if a meeting
    // actually falls outside it.
    const start = Math.min(...all.map((m) => m.startMinutes), 7 * 60);
    const end = Math.max(...all.map((m) => m.endMinutes), 19 * 60);
    // Round out to whole hours so gridlines land cleanly.
    return { start: Math.floor(start / 60) * 60, end: Math.ceil(end / 60) * 60 };
  });

  const hours = createMemo(() => {
    const { start, end } = bounds();
    const list: number[] = [];
    for (let h = start; h < end; h += 60) list.push(h);
    return list;
  });

  return (
    <div class="overflow-x-auto max-w-full">
      <div class="flex min-w-fit">
        <TimeGutter hours={hours()} bounds={bounds()} />
        <For each={ALL_DAYS}>
          {(day) => <DayColumn day={day} hours={hours()} bounds={bounds()} events={events()} />}
        </For>
      </div>
    </div>
  );
}
