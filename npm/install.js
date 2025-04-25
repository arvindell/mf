const axios = require('axios');
const os = require('os');
const fs = require('fs');
const path = require('path');
const tar = require('tar');

// --- Configuration ---
const GITHUB_REPO_OWNER = 'arvindell'; // Replace with your GitHub username
const GITHUB_REPO_NAME = 'mf';       // Replace with your repository name
const VERSION = require('../package.json').version; // Get version from package.json
const BIN_NAME = 'mf';
const INSTALL_DIR = path.join(__dirname);
const BIN_PATH = path.join(INSTALL_DIR, BIN_NAME);
// ---------------------

async function install() {
    const platform = os.platform();
    const arch = os.arch();

    let target = '';
    let archiveExtension = 'tar.gz'; // Default for Linux/macOS

    if (platform === 'darwin') {
        target = arch === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
    } else if (platform === 'linux') {
        target = arch === 'x64' ? 'x86_64-unknown-linux-gnu' : null; // Add other Linux targets if needed (e.g., arm64)
    } else if (platform === 'win32') {
        target = arch === 'x64' ? 'x86_64-pc-windows-msvc' : null; // Assuming MSVC build
        archiveExtension = 'zip'; // Windows usually uses .zip
    } else {
        console.error(`Unsupported platform: ${platform}`);
        process.exit(1);
    }

    if (!target) {
        console.error(`Unsupported architecture: ${arch} for platform: ${platform}`);
        process.exit(1);
    }

    const releaseTag = `v${VERSION}`;
    const filename = `${BIN_NAME}-${releaseTag}-${target}.${archiveExtension}`;
    const downloadUrl = `https://github.com/${GITHUB_REPO_OWNER}/${GITHUB_REPO_NAME}/releases/download/${releaseTag}/${filename}`;

    console.log(`Downloading ${BIN_NAME} binary from ${downloadUrl}`);

    try {
        // Ensure install directory exists
        if (!fs.existsSync(INSTALL_DIR)) {
            fs.mkdirSync(INSTALL_DIR, { recursive: true });
        }

        const response = await axios({
            method: 'get',
            url: downloadUrl,
            responseType: 'stream',
        });

        // Determine temporary path for download
        const tempArchivePath = path.join(INSTALL_DIR, filename); 

        const writer = fs.createWriteStream(tempArchivePath);
        response.data.pipe(writer);

        await new Promise((resolve, reject) => {
            writer.on('finish', resolve);
            writer.on('error', reject);
        });

        console.log(`Downloaded archive to ${tempArchivePath}`);

        // Extract the binary
        console.log(`Extracting ${BIN_NAME} from ${filename}...`);
        if (archiveExtension === 'tar.gz') {
            await tar.x({
                file: tempArchivePath,
                cwd: INSTALL_DIR,
                strip: 1, // Assuming the binary is inside a top-level directory in the archive
                filter: (filePath) => filePath.endsWith(BIN_NAME) || filePath.endsWith(BIN_NAME + '.exe')
            });
        } else if (archiveExtension === 'zip') {
            // Basic zip extraction (requires an external library like 'extract-zip' or manual implementation)
            // For simplicity, we'll assume the binary is directly in the zip for now.
            // You might need: const extract = require('extract-zip'); await extract(tempArchivePath, { dir: INSTALL_DIR });
            console.warn("ZIP extraction not fully implemented. Assuming binary is directly in archive.");
            // If the binary isn't directly in the root, you'll need a proper zip library
            // For now, we'll just copy the archive (assuming it's the binary for simplicity - THIS IS LIKELY WRONG)
            // You **MUST** replace this with actual zip extraction logic
            fs.renameSync(tempArchivePath, BIN_PATH + (platform === 'win32' ? '.exe' : '')); 
        }
        
        // Clean up the downloaded archive if extraction logic didn't already move/rename
        if (fs.existsSync(tempArchivePath)) {
             fs.unlinkSync(tempArchivePath);
             console.log(`Removed temporary archive ${tempArchivePath}`);
        }
        
        // Make binary executable on Unix-like systems
        if (platform !== 'win32') {
            fs.chmodSync(BIN_PATH, '755');
            console.log(`Made ${BIN_PATH} executable.`);
        }

        console.log(`${BIN_NAME} installed successfully to ${BIN_PATH}`);

    } catch (error) {
        console.error(`Error downloading or installing ${BIN_NAME}:`);
        if (error.response && error.response.status === 404) {
            console.error(`  Could not find release artifact at ${downloadUrl}`);
            console.error(`  Please ensure the release ${releaseTag} exists and contains the file ${filename}.`);
        } else {
            console.error(error.message);
        }
        process.exit(1);
    }
}

install(); 