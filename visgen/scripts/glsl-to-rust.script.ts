import { transpileGlslToRust } from "./transpiler/transpile-glsl-to-rust";

// Read from stdin
process.stdin.setEncoding("utf8");
let input = "";

process.stdin.on("data", (chunk) => {
  input += chunk;
});

process.stdin.on("end", () => {
  try {
    const rustCode = transpileGlslToRust(input);
    process.stdout.write(rustCode);
  } catch (error) {
    console.error("Error transpiling GLSL to Rust:", error);
    process.exit(1);
  }
});
