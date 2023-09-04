import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";
import colors from "tailwindcss/colors";

const tailwind_colors = {
    primary: colors.lime,
    secondary: colors.cyan,
    tertiary: colors.purple,
    surface: colors.slate,
    warning: colors.amber,
    error: colors.red,
    success: colors.lime,
};

function hex_to_rgb(hex: string): string {
    let h = hex.slice(1);
    let r = parseInt(h.slice(0, 2), 16);
    let g = parseInt(h.slice(2, 4), 16);
    let b = parseInt(h.slice(4, 6), 16);
    return `${r} ${g} ${b}`;
}

export let properties = {
    "--theme-font-family-base": "system-ui",
    "--theme-font-family-heading": "system-ui",
    "--theme-font-color-base": "0 0 0",
    "--theme-font-color-dark": "255 255 255",
    "--theme-rounded-base": "9999px",
    "--theme-rounded-container": "4px",
    "--theme-border-base": "0px",
    "--on-primary": "255 255 255",
    "--on-secondary": "255 255 255",
    "--on-tertiary": "255 255 255",
    "--on-success": "255 255 255",
    "--on-warning": "255 255 255",
    "--on-error": "255 255 255",
    "--on-surface": "0 0 0",
    "--color-base": "253 253 253",
};

for (const key in tailwind_colors) {
    for (const shade in tailwind_colors[key]) {
        properties[`--color-${key}-${shade}`] = hex_to_rgb(
            tailwind_colors[key][shade]
        );
    }
}

export let theme: CustomThemeConfig = {
    name: "theme",
    properties,
} as any;
