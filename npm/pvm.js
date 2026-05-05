#!/usr/bin/env node
const { spawnSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const bin = path.join(__dirname, "pvm.exe");

if (!fs.existsSync(bin)) {
  console.error("pvm binary not found. Try reinstalling: npm install -g super-pyman");
  process.exit(1);
}

const { status } = spawnSync(bin, process.argv.slice(2), { stdio: "inherit" });
process.exit(status ?? 1);
