#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const { createReadStream, createWriteStream } = require('fs');
const zlib = require('zlib');

const REPO = 'iPeluwa/rush';
const VERSION = 'v0.2.5';

function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;
  
  // Map Node.js platform/arch to GitHub release asset names
  const platformMap = {
    'darwin': {
      'x64': 'rush-x86_64-apple-darwin.tar.gz',
      'arm64': 'rush-x86_64-apple-darwin.tar.gz' // Use x86_64 build for Apple Silicon (works via Rosetta 2)
    },
    'linux': {
      'x64': 'rush-x86_64-unknown-linux-gnu.tar.gz',
      'arm64': 'rush-x86_64-unknown-linux-gnu.tar.gz' // Fallback to x86_64 for now
    },
    'win32': {
      'x64': 'rush-x86_64-pc-windows-msvc.zip'
    }
  };

  if (!platformMap[platform] || !platformMap[platform][arch]) {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }

  return {
    platform,
    arch,
    assetName: platformMap[platform][arch],
    isWindows: platform === 'win32',
    isArchive: true
  };
}

function downloadFile(url, destination) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading TaskRush binary...`);
    console.log(`URL: ${url}`);
    
    const file = fs.createWriteStream(destination);
    
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        return downloadFile(response.headers.location, destination)
          .then(resolve)
          .catch(reject);
      }
      
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: HTTP ${response.statusCode}`));
        return;
      }
      
      response.pipe(file);
      
      file.on('finish', () => {
        file.close();
        resolve();
      });
      
      file.on('error', (err) => {
        fs.unlink(destination, () => {});
        reject(err);
      });
    }).on('error', (err) => {
      fs.unlink(destination, () => {});
      reject(err);
    });
  });
}

function extractArchive(archivePath, extractDir, isWindows) {
  try {
    if (isWindows) {
      // Extract ZIP file
      execSync(`powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${extractDir}'"`, {
        stdio: 'pipe'
      });
    } else {
      // Extract tar.gz file
      execSync(`tar -xzf "${archivePath}" -C "${extractDir}"`, {
        stdio: 'pipe'
      });
    }
  } catch (error) {
    throw new Error(`Failed to extract archive: ${error.message}`);
  }
}

async function install() {
  try {
    const { assetName, isWindows } = getPlatformInfo();
    
    // Create bin directory
    const binDir = path.join(__dirname, 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }
    
    // Create temp directory for extraction
    const tempDir = path.join(__dirname, 'temp');
    if (!fs.existsSync(tempDir)) {
      fs.mkdirSync(tempDir, { recursive: true });
    }
    
    // Download URL for the archive
    const downloadUrl = `https://github.com/${REPO}/releases/download/${VERSION}/${assetName}`;
    
    // Download the archive
    const archivePath = path.join(tempDir, assetName);
    await downloadFile(downloadUrl, archivePath);
    
    console.log('📦 Extracting archive...');
    
    // Extract the archive
    extractArchive(archivePath, tempDir, isWindows);
    
    // Find and copy the binary
    const binaryName = isWindows ? 'taskrush.exe' : 'taskrush';
    const extractedBinaryPath = path.join(tempDir, binaryName);
    const finalBinaryPath = path.join(binDir, binaryName);
    
    if (!fs.existsSync(extractedBinaryPath)) {
      throw new Error(`Binary ${binaryName} not found in extracted archive`);
    }
    
    // Copy binary to final location
    fs.copyFileSync(extractedBinaryPath, finalBinaryPath);
    
    // Make executable on Unix systems
    if (!isWindows) {
      fs.chmodSync(finalBinaryPath, '755');
    }
    
    // Clean up temp directory
    fs.rmSync(tempDir, { recursive: true, force: true });
    
    console.log('✅ TaskRush installed successfully!');
    console.log('');
    console.log('You can now use:');
    console.log('  taskrush --help');
    console.log('  taskrush --list');
    console.log('');
    
  } catch (error) {
    console.error('❌ Failed to install TaskRush:', error.message);
    console.error('');
    console.error('You can install TaskRush manually from:');
    console.error(`https://github.com/${REPO}/releases/tag/${VERSION}`);
    process.exit(1);
  }
}

if (require.main === module) {
  install();
}

module.exports = { install };
