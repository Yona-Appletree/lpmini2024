// Test script for ESP32 with auto-exit on completion
// Usage: npm run test-esp32 [timeout_seconds]

import { spawn } from 'child_process';
import { exit } from 'process';

const timeout: number = parseInt(process.argv[2] || '8');

console.log('Building ESP32 rust-bench...\n');

// Build first
const build = spawn('cargo', ['build', '--bin', 'rust-bench', '--release'], {
  cwd: '../fw-esp32c3',
  stdio: 'pipe',
  env: process.env,
});

let buildOutput = '';
build.stderr.on('data', (data) => {
  buildOutput += data.toString();
});

build.on('close', (code) => {
  if (code !== 0) {
    // Show last 20 lines of build output
    const lines = buildOutput.split('\n');
    console.log(lines.slice(-20).join('\n'));
    console.log('\nBuild failed!');
    exit(1);
  }

  // Build succeeded, now run
  console.log('Running on ESP32...\n');

  const run = spawn('cargo', ['run', '--bin', 'rust-bench', '--release'], {
    cwd: '../fw-esp32c3',
    stdio: 'pipe',
    env: process.env,
  });

  let foundComplete = false;

  run.stdout.on('data', (data: Buffer) => {
    const output = data.toString();
    process.stdout.write(output);

    if (output.includes('Complete')) {
      foundComplete = true;
      setTimeout(() => {
        run.kill('SIGTERM');
      }, 300);
    }
  });

  run.stderr.on('data', (data: Buffer) => {
    process.stderr.write(data);
  });

  // Safety timeout
  const timer = setTimeout(
    () => {
      if (!foundComplete) {
        console.log('\nTimeout reached');
        run.kill('SIGTERM');
      }
    },
    (timeout + 8) * 1000,
  );

  run.on('close', () => {
    clearTimeout(timer);
    console.log('\nDone');
    exit(0);
  });
});
