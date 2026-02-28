#!/usr/bin/env node
const { spawn } = require("child_process");
const path = require("path");

// Look for the binary in the same folder as this script
const binaryPath = path.join(__dirname, "icons-master.exe");

const args = process.argv.slice(2);
const child = spawn(binaryPath, args, { stdio: "inherit" });

child.on("exit", (code) => {
  process.exit(code || 0);
});

child.on("error", (err) => {
  console.error(
    `Error: Could not find or run the binary at ${binaryPath}. Did you run 'npm run build' first?`
  );
  process.exit(1);
});
