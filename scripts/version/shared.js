const path = require("path");
const fs = require("fs");
const TOML = require("@iarna/toml");

/**
 *
 * @param {string} version Current version
 * @param {string} type Type of version bump
 * @returns {string} New version
 */
function bump(version, type) {
  let versionParts = version.split(".");
  if (versionParts.length !== 3) {
    console.error(`Invalid version ${version}.`);
    process.exit(1);
  }

  switch (type) {
    case "major": {
      let majorVersion = parseInt(versionParts[0], 10);
      majorVersion += 1;
      versionParts[0] = majorVersion.toString();
      break;
    }
    case "minor": {
      let minorVersion = parseInt(versionParts[1], 10);
      minorVersion += 1;
      versionParts[1] = minorVersion.toString();
      break;
    }
    case "patch": {
      let patchVersion = parseInt(versionParts[2], 10);
      patchVersion += 1;
      versionParts[2] = patchVersion.toString();
      break;
    }
    default:
      console.error(`Invalid version type ${type}`);
      process.exit(1);
  }

  return versionParts.join(".");
}

/**
 *
 * @param {string} tomlStr Toml string
 * @param {string} newVersion New version
 * @returns {string} Toml string with new version
 */
function replaceTomlVersion(tomlStr, newVersion) {
  const regex = /version\s*=\s*"([0-9]+\.[0-9]+\.[0-9]+)"/;
  return tomlStr.replace(regex, `version = "${newVersion}"`);
}

/**
 *
 * @param {string} pkgJsonStr Package.json string
 * @param {string} newVersion New version
 * @returns {string} Package.json string with new version
 */
function replacePkgJsonVersion(pkgJsonStr, newVersion) {
  const regex = /"version"\s*:\s*"([^"]*)"/;
  return pkgJsonStr.replace(regex, `"version": "${newVersion}"`);
}

/**
 * Bump version of all packages
 * @param {string} type Type of version bump
 */
module.exports = function (type) {
  const cwd = process.cwd();

  const pkgsPath = path.join(cwd, "packages");

  const corePkgPath = path.join(pkgsPath, "core");
  const cargoTomlPath = path.join(corePkgPath, "Cargo.toml");

  const cargoTomlStr = fs.readFileSync(cargoTomlPath, "utf8");
  const cargoToml = TOML.parse(cargoTomlStr);

  const currentVersion = cargoToml.package.version;
  if (!currentVersion) {
    console.error(`No version found in ${cargoTomlPath}`);
    process.exit(1);
  }

  const newVersion = bump(currentVersion, type);

  const newCargoTomlStr = replaceTomlVersion(cargoTomlStr, newVersion);
  fs.writeFileSync(cargoTomlPath, newCargoTomlStr, "utf8");

  const pkgs = fs.readdirSync(pkgsPath);

  for (const pkg of pkgs) {
    if (pkg.startsWith(".") || pkg === "core") {
      continue;
    }

    const pkgPath = path.join(pkgsPath, pkg);

    if (!fs.statSync(pkgPath).isDirectory()) {
      continue;
    }

    const pkgJsonPath = path.join(pkgPath, "package.json");
    const pkgJsonStr = fs.readFileSync(pkgJsonPath, "utf8");
    const newPkgJsonStr = replacePkgJsonVersion(pkgJsonStr, newVersion);

    fs.writeFileSync(pkgJsonPath, newPkgJsonStr, "utf8");
  }

  const rootPkgJsonPath = path.join(cwd, "package.json");
  const rootPkgJsonStr = fs.readFileSync(rootPkgJsonPath, "utf8");
  const newRootPkgJsonStr = replacePkgJsonVersion(rootPkgJsonStr, newVersion);

  fs.writeFileSync(rootPkgJsonPath, newRootPkgJsonStr, "utf8");

  console.log(`Bumped version from ${currentVersion} to ${newVersion}`);
};
