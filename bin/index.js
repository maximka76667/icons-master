#!/usr/bin/env node
const { spawn } = require("child_process");
const path = require("path");
const os = require("os");

// Detect if we are on Windows or Unix-like
const isWindows = os.platform() === "win32";
const binaryName = isWindows ? "icons-manager.exe" : "icons-manager";

// Point to the compiled binary in the target folder
const binaryPath = path.join(__dirname, "..", "target", "release", binaryName);

// Forward all command line arguments to the Rust binary
const args = process.argv.slice(2);
const child = spawn(binaryPath, args, { stdio: "inherit" });

child.on("exit", (code) => {
  process.exit(code || 0);
});

child.on("error", (err) => {
  console.error(`Failed to start icons-manager: ${err.message}`);
  process.exit(1);
});
