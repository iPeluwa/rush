// Main entry point for programmatic usage
const path = require('path');
const { spawn } = require('child_process');
const fs = require('fs');

/**
 * Run TaskRush programmatically
 * @param {string[]} args - Arguments to pass to TaskRush
 * @param {Object} options - Spawn options
 * @returns {Promise<number>} - Exit code
 */
function runTaskRush(args = [], options = {}) {
  return new Promise((resolve, reject) => {
    const isWindows = process.platform === 'win32';
    const binaryName = isWindows ? 'taskrush.exe' : 'taskrush';
    const binaryPath = path.join(__dirname, 'bin', binaryName);

    if (!fs.existsSync(binaryPath)) {
      reject(new Error('TaskRush binary not found. Try reinstalling the package.'));
      return;
    }

    const defaultOptions = {
      stdio: 'inherit',
      cwd: process.cwd(),
      ...options
    };

    const child = spawn(binaryPath, args, defaultOptions);

    child.on('close', (code) => {
      resolve(code);
    });

    child.on('error', (err) => {
      reject(err);
    });
  });
}

/**
 * Get the path to the TaskRush binary
 * @returns {string} - Path to the binary
 */
function getBinaryPath() {
  const isWindows = process.platform === 'win32';
  const binaryName = isWindows ? 'taskrush.exe' : 'taskrush';
  return path.join(__dirname, 'bin', binaryName);
}

module.exports = {
  runTaskRush,
  getBinaryPath
};
