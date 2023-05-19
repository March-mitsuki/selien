const os = require("os");

const platform = os.platform();
const arch = os.arch();

if (platform !== "darwin" || arch !== "arm64") {
  console.error("Error: aarch64-apple-darwin does not support cross compile.");
  process.exit(1);
}
