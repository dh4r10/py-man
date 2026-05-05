const https = require("https");
const fs = require("fs");
const path = require("path");

const VERSION = require("./package.json").version;
const BINARY = path.join(__dirname, "pvm.exe");
const REPO = "dh4r10/py-man";
const URL = `https://github.com/${REPO}/releases/download/v${VERSION}/pvm.exe`;

if (process.platform !== "win32") {
  console.error("super-pyman: only Windows is supported at this time.");
  process.exit(1);
}

if (fs.existsSync(BINARY)) {
  process.exit(0);
}

console.log(`Downloading pvm v${VERSION}...`);

function download(url, dest, redirects = 5) {
  if (redirects === 0) {
    console.error("Too many redirects.");
    process.exit(1);
  }

  https.get(url, { headers: { "User-Agent": "super-pyman-installer" } }, (res) => {
    if (res.statusCode === 301 || res.statusCode === 302) {
      return download(res.headers.location, dest, redirects - 1);
    }
    if (res.statusCode !== 200) {
      console.error(`Failed to download pvm.exe: HTTP ${res.statusCode}`);
      console.error(`URL: ${url}`);
      process.exit(1);
    }

    const file = fs.createWriteStream(dest);
    res.pipe(file);
    file.on("finish", () => {
      file.close();
      console.log("pvm installed successfully.");
      console.log("");
      console.log("To activate PVM in PowerShell, add this to your $PROFILE:");
      console.log("  pvm env | Out-String | Invoke-Expression");
    });
  }).on("error", (err) => {
    fs.unlink(dest, () => {});
    console.error(`Download error: ${err.message}`);
    process.exit(1);
  });
}

download(URL, BINARY);
