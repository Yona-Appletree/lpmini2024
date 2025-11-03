#!/usr/bin/env node --experimental-strip-types
"use strict";
// Test script for ESP32 with auto-exit on completion
// Usage: ./test_on_esp32.ts [timeout_seconds]
Object.defineProperty(exports, "__esModule", { value: true });
var child_process_1 = require("child_process");
var process_1 = require("process");
var timeout = parseInt(process.argv[2] || '8');
console.log('Building ESP32 rust-bench...\n');
// Build first
var build = (0, child_process_1.spawn)('cargo', ['build', '--bin', 'rust-bench', '--release'], {
    cwd: 'fw-esp32c3',
    stdio: 'pipe'
});
var buildOutput = '';
build.stderr.on('data', function (data) {
    buildOutput += data.toString();
});
build.on('close', function (code) {
    if (code !== 0) {
        // Show last 20 lines of build output
        var lines = buildOutput.split('\n');
        console.log(lines.slice(-20).join('\n'));
        console.log('\nBuild failed!');
        (0, process_1.exit)(1);
    }
    // Build succeeded, now run
    console.log('Running on ESP32...\n');
    var run = (0, child_process_1.spawn)('cargo', ['run', '--bin', 'rust-bench', '--release'], {
        cwd: 'fw-esp32c3',
        stdio: 'pipe'
    });
    var foundComplete = false;
    run.stdout.on('data', function (data) {
        var output = data.toString();
        process.stdout.write(output);
        if (output.includes('Complete')) {
            foundComplete = true;
            setTimeout(function () {
                run.kill('SIGTERM');
            }, 300);
        }
    });
    run.stderr.on('data', function (data) {
        process.stderr.write(data);
    });
    // Safety timeout
    var timer = setTimeout(function () {
        if (!foundComplete) {
            console.log('\nTimeout reached');
            run.kill('SIGTERM');
        }
    }, (timeout + 8) * 1000);
    run.on('close', function () {
        clearTimeout(timer);
        console.log('\nDone');
        (0, process_1.exit)(0);
    });
});
