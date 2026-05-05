const https = require('https');
const fs = require('fs');
const path = require('path');

const c = {
  reset:   '\x1b[0m',
  bold:    '\x1b[1m',
  purple:  '\x1b[35m',
  violet:  '\x1b[95m',
  dim:     '\x1b[2m',
  cyan:    '\x1b[96m',
};
const p = (code, text) => `${code}${text}${c.reset}`;

const VERSION = require('./package.json').version;
const BINARY = path.join(__dirname, 'pvm.exe');
const REPO = 'dh4r10/py-man';
const URL = `https://github.com/${REPO}/releases/download/v${VERSION}/pvm.exe`;

if (process.platform !== 'win32') {
  console.error('super-pyman: only Windows is supported at this time.');
  process.exit(1);
}

if (fs.existsSync(BINARY)) {
  process.exit(0);
}

console.log(p(c.violet + c.bold, `Downloading pvm v${VERSION}...`));

function download(url, dest, redirects = 5) {
  if (redirects === 0) {
    console.error('Too many redirects.');
    process.exit(1);
  }

  https
    .get(url, { headers: { 'User-Agent': 'super-pyman-installer' } }, (res) => {
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
      file.on('finish', () => {
        file.close();
        console.log(p(c.violet + c.bold, '✓ pvm installed successfully.'));
        console.log('');
        console.log(p(c.purple + c.bold, 'Recommendations:'));
        console.log('');
        console.log(p(c.dim, 'To activate PVM, add this to your PowerShell $PROFILE:'));
        console.log(p(c.cyan, '  pvm env | Out-String | Invoke-Expression'));
        console.log('');
        console.log(p(c.dim, 'Then restart your terminal for the changes to take effect.'));
      });
    })
    .on('error', (err) => {
      fs.unlink(dest, () => {});
      console.error(`Download error: ${err.message}`);
      process.exit(1);
    });
}

download(URL, BINARY);
