#!/usr/bin/env node
const { spawn } = require("child_process");
const path = require("path");
const fs = require("fs");
const https = require("https");
const { version } = require("../package.json");

const REPO = "maximka76667/icons-master";

function getBinaryName() {
  const { platform, arch } = process;
  if (platform === "win32") return "icons-master-windows-x64.exe";
  if (platform === "darwin" && arch === "arm64") return "icons-master-darwin-arm64";
  if (platform === "darwin") return "icons-master-darwin-x64";
  if (platform === "linux" && arch === "arm64") return "icons-master-linux-arm64";
  if (platform === "linux") return "icons-master-linux-x64";
  throw new Error(`Unsupported platform: ${platform} ${arch}`);
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const follow = (url) => {
      https.get(url, (res) => {
        if (res.statusCode === 301 || res.statusCode === 302) {
          return follow(res.headers.location);
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`Failed to download binary: HTTP ${res.statusCode}`));
        }
        const file = fs.createWriteStream(dest);
        res.pipe(file);
        file.on("finish", () => file.close(resolve));
        file.on("error", reject);
      }).on("error", reject);
    };
    follow(url);
  });
}

async function main() {
  const name = getBinaryName();
  const binaryPath = path.join(__dirname, name);

  if (!fs.existsSync(binaryPath)) {
    const url = `https://github.com/${REPO}/releases/download/v${version}/${name}`;
    process.stderr.write(`Downloading icons-master v${version}...\n`);
    await download(url, binaryPath);
    if (process.platform !== "win32") {
      fs.chmodSync(binaryPath, 0o755);
    }
  }

  const child = spawn(binaryPath, process.argv.slice(2), { stdio: "inherit" });
  child.on("exit", (code) => process.exit(code || 0));
  child.on("error", () => {
    console.error(`Error: Could not run binary at ${binaryPath}`);
    process.exit(1);
  });
}

main().catch((err) => {
  console.error(`Error: ${err.message}`);
  process.exit(1);
});
