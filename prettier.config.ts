import type { Config } from "prettier";
import svelte from "prettier-plugin-svelte";

const config: Config = {
    plugins: [svelte],
    trailingComma: "es5",
    tabWidth: 4,
    overrides: [
        {
            files: "*.svelte",
            options: {
                parser: "svelte",
            },
        },
    ],
};

export default config;
