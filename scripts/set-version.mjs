import fs from "node:fs";

const args = process.argv.slice(2);
const disableMacosSigning = args.includes("--disable-macos-signing");
const versionArg = args.find((arg) => !arg.startsWith("--"));

if (!versionArg) {
  console.error("Usage: node scripts/set-version.mjs <version> [--disable-macos-signing]");
  process.exit(1);
}

const version = versionArg.trim().replace(/^[vV]/, "");
if (!version) {
  console.error("Version cannot be empty.");
  process.exit(1);
}

const packageJsonPath = new URL("../package.json", import.meta.url);
const packageLockPath = new URL("../package-lock.json", import.meta.url);
const tauriConfigPath = new URL("../src-tauri/tauri.conf.json", import.meta.url);
const tauriLinuxConfigPath = new URL("../src-tauri/tauri.linux.conf.json", import.meta.url);
const cargoTomlPath = new URL("../src-tauri/Cargo.toml", import.meta.url);
const cargoLockPath = new URL("../src-tauri/Cargo.lock", import.meta.url);

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
packageJson.version = version;
fs.writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`);

const packageLock = JSON.parse(fs.readFileSync(packageLockPath, "utf8"));
packageLock.version = version;
if (packageLock.packages?.[""]) {
  packageLock.packages[""].version = version;
}
fs.writeFileSync(packageLockPath, `${JSON.stringify(packageLock, null, 2)}\n`);

const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, "utf8"));
tauriConfig.version = version;
if (disableMacosSigning) {
  tauriConfig.bundle = tauriConfig.bundle ?? {};
  tauriConfig.bundle.macOS = tauriConfig.bundle.macOS ?? {};
  tauriConfig.bundle.macOS.signingIdentity = null;
}
fs.writeFileSync(tauriConfigPath, `${JSON.stringify(tauriConfig, null, 2)}\n`);

const tauriLinuxConfig = JSON.parse(fs.readFileSync(tauriLinuxConfigPath, "utf8"));
tauriLinuxConfig.productName = tauriConfig.productName;
tauriLinuxConfig.version = version;
fs.writeFileSync(tauriLinuxConfigPath, `${JSON.stringify(tauriLinuxConfig, null, 2)}\n`);

const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
const nextCargoToml = cargoToml.replace(/^version = "[^"]*"/m, `version = "${version}"`);
if (nextCargoToml !== cargoToml) {
  fs.writeFileSync(cargoTomlPath, nextCargoToml);
}

const cargoLock = fs.readFileSync(cargoLockPath, "utf8");
const nextCargoLock = cargoLock.replace(
  /(\[\[package\]\]\nname = "mininote"\nversion = ")[^"]*(")/,
  `$1${version}$2`,
);
if (nextCargoLock !== cargoLock) {
  fs.writeFileSync(cargoLockPath, nextCargoLock);
}

console.log(
  `Synced package.json, package-lock.json, src-tauri/tauri.conf.json, src-tauri/tauri.linux.conf.json, src-tauri/Cargo.toml, and src-tauri/Cargo.lock to ${version}.`,
);
