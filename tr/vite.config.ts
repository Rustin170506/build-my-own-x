import { defineConfig } from "vitest/config";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
  test: {
    include: ["**/*.ts"],
    passWithNoTests: true,
    environment: "jsdom",
  },
  plugins: [tsconfigPaths()],
});
