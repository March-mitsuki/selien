const { spawn } = require("child_process");
const path = require("path");

if (process.platform != "win32") {
  console.error("Error: This script is only for Windows");
  process.exit(1);
}

const args = process.argv.slice(2);
const hyphenIdx = args.indexOf("--");
if (hyphenIdx == -1) {
  console.error("Error: You must passed '--'");
  process.exit(1);
}
const selienArgs = args.slice(hyphenIdx + 1);

const cwd = process.cwd();
const selienRoot = path.join(cwd, "packages", "core");
const build = () => {
  return spawn(
    "powershell",
    ["-Command", `cd ${selienRoot}; if ($?) { cargo build --bin selien }`],
    { shell: true, stdio: "inherit" }
  );
};

const selienTest = path.join(selienRoot, "tests");
const selienDebug = path.join(selienRoot, "target", "debug", "selien.exe");
const dev = () => {
  return spawn(
    "powershell",
    [
      "-Command",
      `cd ${selienTest}; if ($?) { ${selienDebug} ${selienArgs.join(" ")} }`,
    ],
    {
      shell: true,
      stdio: "inherit",
    }
  );
};

build().on("close", (code) => {
  dev();
});
