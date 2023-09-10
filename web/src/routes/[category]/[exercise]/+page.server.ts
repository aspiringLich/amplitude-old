import { getExercise } from "$lib/fetch";
import type { ExerciseData } from "$lib/fetch";

export const load = async ({ params, fetch }): Promise<ExerciseData> => {
    return getExercise(`${params.category}/${params.exercise}`);
};

// export const prerender = true;
// export const csr = false;
