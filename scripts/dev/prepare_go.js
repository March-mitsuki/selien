const fs = require("fs");
const path = require("path");

const goVersion = "1.20";
const goModName = "selien";
console.log(
  `Preparing go dev environment, with version: ${goVersion}, mod name: ${goModName}`
);

function createFileRecurs(filePath, content) {
  var dirname = path.dirname(filePath);
  if (!fs.existsSync(dirname)) {
    fs.mkdirSync(dirname, { recursive: true });
  }

  fs.writeFileSync(filePath, content);
}

const cwd = process.cwd();
const devGoWork = `go ${goVersion}

use (
  ./packages/core/tests/dist/packages/go
)
`;
const devGoWorkPath = path.join(cwd, "go.work");
createFileRecurs(devGoWorkPath, devGoWork);

const __devGoRootPath = path.join(
  cwd,
  "packages",
  "core",
  "tests",
  "dist",
  "packages",
  "go"
);
const devGoMod = `module ${goModName}

go 1.20
`;
const devGoModPath = path.join(__devGoRootPath, "go.mod");
createFileRecurs(devGoModPath, devGoMod);

const testApiFile = `package types

type Css = interface{}
`;
const testApiFilePath = path.join(__devGoRootPath, "api", "types", "api.go");
createFileRecurs(testApiFilePath, testApiFile);
