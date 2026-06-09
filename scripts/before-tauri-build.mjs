import { spawnSync } from "node:child_process";

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    stdio: "inherit",
    shell: process.platform === "win32",
    ...options,
  });

  if (result.error) {
    throw result.error;
  }

  return result.status ?? 0;
}

if (process.platform === "win32") {
  for (const processName of ["mininote.exe", "MiniNote.exe"]) {
    run("taskkill", ["/F", "/IM", processName], { stdio: "ignore" });
  }
}

const npmCommand = process.platform === "win32" ? "npm.cmd" : "npm";
const status = run(npmCommand, ["run", "build"]);
process.exit(status);
