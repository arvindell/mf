#!/usr/bin/env node

const path = require('path');
const cp = require('child_process');
const os = require('os');

const binName = 'mf' + (os.platform() === 'win32' ? '.exe' : '');
const binPath = path.join(__dirname, binName);
const args = process.argv.slice(2); // Pass arguments from npm script to the binary

// Execute the binary
try {
    // Use stdio: 'inherit' to pass through stdin/stdout/stderr
    const result = cp.spawnSync(binPath, args, { stdio: 'inherit' });

    // Exit with the same code as the binary process
    process.exit(result.status);
} catch (error) {
    console.error(`Error executing ${binName}:`, error);
    process.exit(1);
} 