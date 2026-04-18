const { existsSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process;

// Platform mapping: platform package name -> local .node file suffix
const platformMap = {
  'linux': {
    'x64': { pkg: 'ruvector-core-linux-x64-gnu', file: 'ruvector.linux-x64-gnu.node' },
    'arm64': { pkg: 'ruvector-core-linux-arm64-gnu', file: 'ruvector.linux-arm64-gnu.node' }
  },
  'darwin': {
    'x64': { pkg: 'ruvector-core-darwin-x64', file: 'ruvector.darwin-x64.node' },
    'arm64': { pkg: 'ruvector-core-darwin-arm64', file: 'ruvector.darwin-arm64.node' }
  },
  'win32': {
    'x64': { pkg: 'ruvector-core-win32-x64-msvc', file: 'ruvector.win32-x64-msvc.node' }
  }
};

let nativeBinding = null
let loadError = null

function loadNativeModule() {
  const entry = platformMap[platform]?.[arch];

  if (!entry) {
    throw new Error(
      `Unsupported platform: ${platform}-${arch}\n` +
      `Ruvector native module is available for:\n` +
      `- Linux (x64, ARM64)\n` +
      `- macOS (x64, ARM64)\n` +
      `- Windows (x64)`
    );
  }

  // Try local file first (bundled binary — ADR-0071)
  const localFile = join(__dirname, entry.file)
  const localFileExisted = existsSync(localFile)
  try {
    if (localFileExisted) {
      nativeBinding = require(localFile)
    } else {
      nativeBinding = require(entry.pkg)
    }
  } catch (e) {
    loadError = e
  }

  if (!nativeBinding) {
    if (loadError) {
      if (loadError.code === 'MODULE_NOT_FOUND') {
        throw new Error(
          `Native module not found for ${platform}-${arch}\n` +
          `Please install: npm install ${entry.pkg}\n` +
          `Or reinstall ruvector-core to get optional dependencies`
        );
      }
      throw loadError;
    }
    throw new Error(`Failed to load native binding for ${platform}-${arch}`);
  }

  return nativeBinding;
}

module.exports = loadNativeModule();
