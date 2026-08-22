import { createMemo } from "solid-js";

const COURSE_TO_SYLLABUS: Record<string, string> = {
    "MATE3005": "https://www.uprm.edu/math/wp-content/uploads/sites/170/2023/02/Mate3005.pdf",
    "CIIC3015": "https://www.uprm.edu/cse/wp-content/uploads/sites/153/2020/03/CIIC-3015-Introduction-to-Computer-Programming-I.pdf",
};

export function CourseLink(props: { course: string; text?: string; class?: string }) {
    const link = createMemo(() => COURSE_TO_SYLLABUS[props.course] ?? "#");
    const classes = createMemo(() => props.class ?? "link link-primary");

    return (
        <a href={link()} class={classes()}>
            {props.text ?? props.course}
        </a>
    );
}