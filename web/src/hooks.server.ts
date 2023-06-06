import colors from "tailwindcss/colors";
import fs from "fs";
import path from "path";

let map = {
    primary: colors.lime,
    secondary: colors.cyan,
    tertiary: colors.purple,
    surface: colors.slate,
    warning: colors.amber,
    error: colors.red,
    success: colors.lime,
};

function skeletonColor(hex: string): string {
    let h = hex.slice(1);
    let r = parseInt(h.slice(0, 2), 16);
    let g = parseInt(h.slice(2, 4), 16);
    let b = parseInt(h.slice(4, 6), 16);
    return `${r} ${g} ${b}`;
}

let content = "";

for (const [shade, tw_colors] of Object.entries(map)) {
    for (const [level, color] of Object.entries(tw_colors)) {
        content += `    --color-${shade}-${level}: ${skeletonColor(color)};\n`;
    }
}

fs.writeFileSync(path.resolve("./src/themes/colors.postcss"), `
:root {
    ${content.trim()}
}
`.trim());

