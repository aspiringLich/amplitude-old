import { fetchCourseList } from "$lib/fetch";
import type { ParamMatcher } from "@sveltejs/kit";

const course_list = await fetchCourseList();
const courses = new Set(Object.keys(course_list));

export const match = ((param) => {
    return courses.has(param);
}) satisfies ParamMatcher;