import { parse } from "@shaderfrog/glsl-parser";
import type {
  DeclarationStatementNode,
  DeclaratorListNode,
  FunctionNode,
  KeywordNode,
  Program,
} from "@shaderfrog/glsl-parser/ast";
import { typeMap } from "./glsl-type-to-rust";

interface ShaderInfo {
  uniforms: { [key: string]: { type: string; name: string } };
  inputs: { [key: string]: { type: string; name: string } };
  outputs: { [key: string]: { type: string; name: string } };
  functions: {
    [key: string]: {
      name: string;
      code: string;
    };
  };
}

export function transpileGlslToRust(glsl: string): string {
  const ast = parse(glsl);

  // Extract uniforms, inputs, and outputs
  const shaderInfo: ShaderInfo = {
    uniforms: {},
    inputs: {},
    outputs: {},
    functions: {},
  };

  handleProgram(shaderInfo, ast);

  // Generate Rust code
  return generateRustCode(shaderInfo);
}

function handleProgram(shaderInfo: ShaderInfo, program: Program) {
  for (const node of program.program) {
    switch (node.type) {
      case "declaration_statement":
        handleDeclarationStatement(
          shaderInfo,
          node as DeclarationStatementNode,
        );
        break;
      case "function":
        handleFunction(shaderInfo, node as FunctionNode);
        break;
    }
  }
}

function handleDeclarationStatement(
  shaderInfo: ShaderInfo,
  node: DeclarationStatementNode,
) {
  if (node.declaration.type !== "declarator_list") return;

  const declaratorList = node.declaration as DeclaratorListNode;
  const typeSpec = (
    declaratorList.specified_type.specifier.specifier as KeywordNode<string>
  ).token;
  const rustType = typeMap[typeSpec] || typeSpec;

  // Get qualifiers (uniform, in, out)
  const qualifiers =
    declaratorList.specified_type.qualifiers?.map(
      (q) => (q as KeywordNode<string>).token,
    ) || [];

  for (const declaration of declaratorList.declarations) {
    const name = declaration.identifier.identifier;

    if (qualifiers.includes("uniform")) {
      shaderInfo.uniforms[name] = { type: rustType, name };
    } else if (qualifiers.includes("in")) {
      shaderInfo.inputs[name] = { type: rustType, name };
    } else if (qualifiers.includes("out")) {
      shaderInfo.outputs[name] = { type: rustType, name };
    }
  }
}

function handleFunction(shaderInfo: ShaderInfo, node: FunctionNode) {
  const name = node.prototype.header.name.identifier;
  shaderInfo.functions[name] = {
    name,
    code: generateFunctionCode(node),
  };
}

function generateFunctionCode(_node: FunctionNode): string {
  // TODO: Implement function code generation
  return "";
}

function generateRustCode(shaderInfo: ShaderInfo): string {
  const inputFields = Object.entries(shaderInfo.inputs)
    .map(([name, info]) => `    pub ${name}: ${info.type},`)
    .join("\n");

  const outputFields = Object.entries(shaderInfo.outputs)
    .map(([name, info]) => `    pub ${name}: ${info.type},`)
    .join("\n");

  const uniformFields = Object.entries(shaderInfo.uniforms)
    .map(([name, info]) => `    pub ${name}: ${info.type},`)
    .join("\n");

  return `
pub struct ShaderInput {
${inputFields}
${uniformFields}
}

pub struct ShaderOutput {
${outputFields}
}

pub fn main(
    ShaderInput { ${Object.keys(shaderInfo.inputs).concat(Object.keys(shaderInfo.uniforms)).join(", ")} }: ShaderInput
) -> ShaderOutput {
    // TODO: Implement main function body
    unimplemented!()
}`;
}
