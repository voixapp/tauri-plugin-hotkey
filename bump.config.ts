import { defineConfig } from "bumpp";

export default defineConfig({
  commit: true,
  tag: true,
  push: true,
  files: ["package.json", "Cargo.toml"],
  all: true,
});
