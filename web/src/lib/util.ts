/**
 * Reduce calls to the passed function.
 *
 * @param func - Function to debounce.
 * @param threshold - The delay to avoid recalling the function.
 * @param execAsap - If true, the Function is called at the start of the threshold, otherwise the Function is called at the end of the threshold.
 */
export function debounce<T extends (...args: any[]) => any>(
    func: T,
    threshold: number,
    execAsap = false
): T {
    let timeout: any;

    return function debounced(this: any, ...args: any[]): any {
        const self = this;

        if (timeout) clearTimeout(timeout);
        else if (execAsap) func.apply(self, args);

        timeout = setTimeout(delayed, threshold || 100);

        function delayed(): void {
            if (!execAsap) func.apply(self, args);
            timeout = null;
        }
    } as T;
}

export function mod(n: number, m: number): number {
    return ((n % m) + m) % m;
}

// https://stackoverflow.com/questions/7225407/convert-camelcasetext-to-title-case-text
export const camelToTitle = (str: string) => {
    return str
        .replace(/([A-Z])/g, (match) => ` ${match}`)
        .replace(/^./, (match) => match.toUpperCase())
        .trim();
};
// https://stackoverflow.com/questions/54246477/how-to-convert-camelcase-to-snake-case
export const camelToSkewer = (str) =>
    str.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);

