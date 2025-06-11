import { TextureDef } from "@/core/data/types/texture-def.tsx";
import { FloatDef } from "@/core/data/types/float-def.tsx";
import { RecordDef } from "@/core/data/types/record-def.tsx";
import { glsl } from "@/frontend/util/glsl.ts";
import { defineModule } from "@/core/program/define-module.ts";
import { Gl2dFragmentShader } from "@/core/gl2d/gl2d-fragment-shader.ts";
import type { RuntimeContext } from "@/core/program/program-runtime.ts";
import { useEffect, useRef } from "react";
import { Vec2Def } from "@/core/data/types/vec2-def.tsx";
import { Vec3Def } from "@/core/data/types/vec3-def.tsx";
import { Gl2dFramebuffer } from "@/core/gl2d/gl2d-framebuffer.ts";

export const GlFluidModule = defineModule(
  "gl-fluid",
  {
    label: "Fluid Simulation",
    input: RecordDef({
      viscosity: FloatDef({ default: 0.0005 }),
      diffusion: FloatDef({ default: 0.0002 }),
      velocityDissipation: FloatDef({ default: 0.98 }),
      densityDissipation: FloatDef({ default: 0.98 }),
      deltaTime: FloatDef({ default: 0.016 }),
      emitterLocation: Vec2Def({ default: [0.5, 0.5] }),
      emitterDirection: Vec2Def({ default: [0.0, 1.0] }),
      emitterStrength: FloatDef({ default: 0.2 }),
      emitterColor: Vec3Def({ default: [1.0, 0.5, 0.2] }), // RGB color
    }),
    output: TextureDef(),
  },
  ({ gl2d }) => {
    // Create shader for combined fluid simulation
    const fluidShader = Gl2dFragmentShader(gl2d.context, fluidStepGlsl.trim());

    // Create shader for rendering
    const renderShader = Gl2dFragmentShader(gl2d.context, renderGlsl.trim());

    // Create shader for initialization
    const initShader = Gl2dFragmentShader(gl2d.context, initShaderGlsl.trim());

    // Create framebuffers for double buffering simulation state for each color
    const redBuffer1 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });
    const redBuffer2 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });
    const greenBuffer1 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });
    const greenBuffer2 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });
    const blueBuffer1 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });
    const blueBuffer2 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });

    // Initialize all framebuffers
    [
      redBuffer1,
      redBuffer2,
      greenBuffer1,
      greenBuffer2,
      blueBuffer1,
      blueBuffer2,
    ].forEach((buffer) => {
      buffer.bind();
      initShader.draw({});
    });

    // Create framebuffer for final rendering
    const renderBuffer = gl2d.framebuffer({
      options: { filter: "linear" },
    });

    function Component(props: { context: RuntimeContext }) {
      const canvasRef = useRef<HTMLCanvasElement | null>(null);

      useEffect(() => {
        return props.context.addTickHandler(() => {
          const ctx = canvasRef.current?.getContext("2d");
          if (ctx) {
            renderBuffer.texture.drawToScreen();
            ctx.drawImage(gl2d.canvas, 0, 0);
          }
        });
      }, [props.context]);

      return (
        <div>
          <canvas
            ref={canvasRef}
            width={gl2d.canvas.width}
            height={gl2d.canvas.height}
          />
        </div>
      );
    }

    let frame = 0;

    // Helper function to update a single color channel
    function updateColorChannel(
      readBuffer: Gl2dFramebuffer,
      writeBuffer: Gl2dFramebuffer,
      emitterDensity: number,
      input: {
        deltaTime: number;
        viscosity: number;
        diffusion: number;
        velocityDissipation: number;
        densityDissipation: number;
        emitterLocation: [number, number];
        emitterDirection: [number, number];
        emitterStrength: number;
      }
    ) {
      writeBuffer.bind();
      fluidShader.draw({
        u_fluid: { type: "texture", value: readBuffer.texture },
        u_deltaTime: { type: "float32", value: input.deltaTime },
        u_viscosity: { type: "float32", value: input.viscosity },
        u_diffusion: { type: "float32", value: input.diffusion },
        u_dissipation: { type: "float32", value: input.velocityDissipation },
        u_densityDissipation: {
          type: "float32",
          value: input.densityDissipation,
        },
        u_emitterPos: { type: "vec2", value: input.emitterLocation },
        u_emitterDir: { type: "vec2", value: input.emitterDirection },
        u_emitterStrength: { type: "float32", value: input.emitterStrength },
        u_emitterDensity: { type: "float32", value: emitterDensity },
      });
      return writeBuffer.texture;
    }

    return {
      update: ({ input }) => {
        // Update each color channel
        const redRead = frame % 2 === 0 ? redBuffer1 : redBuffer2;
        const redWrite = frame % 2 === 0 ? redBuffer2 : redBuffer1;
        const redTexture = updateColorChannel(
          redRead,
          redWrite,
          input.emitterColor[0],
          input
        );

        const greenRead = frame % 2 === 0 ? greenBuffer1 : greenBuffer2;
        const greenWrite = frame % 2 === 0 ? greenBuffer2 : greenBuffer1;
        const greenTexture = updateColorChannel(
          greenRead,
          greenWrite,
          input.emitterColor[1],
          input
        );

        const blueRead = frame % 2 === 0 ? blueBuffer1 : blueBuffer2;
        const blueWrite = frame % 2 === 0 ? blueBuffer2 : blueBuffer1;
        const blueTexture = updateColorChannel(
          blueRead,
          blueWrite,
          input.emitterColor[2],
          input
        );

        // Combine color channels in the render buffer
        renderBuffer.bind();
        renderShader.draw({
          u_fluidR: { type: "texture", value: redTexture },
          u_fluidG: { type: "texture", value: greenTexture },
          u_fluidB: { type: "texture", value: blueTexture },
        });

        frame++;
        return renderBuffer.texture;
      },
      component: Component,
    };
  }
);

const initShaderGlsl = glsl`
#version 300 es
precision highp float;
out vec4 fragColor;

void main() {
    // Initialize with zero velocity, zero density, and zero pressure
    fragColor = vec4(0.00, 0.00, 0.0, 0.0);
}
`;

const fluidStepGlsl = glsl`
#version 300 es
precision highp float;

uniform sampler2D u_fluid;
uniform float u_deltaTime;
uniform float u_viscosity;
uniform float u_diffusion;
uniform float u_dissipation;
uniform float u_densityDissipation;
uniform vec2 u_emitterPos;
uniform vec2 u_emitterDir;
uniform float u_emitterStrength;
uniform float u_emitterDensity;

out vec4 fragColor;

// Helper function to get fluid cell index with proper boundary handling
vec4 sampleFluid(sampler2D tex, ivec2 pos) {
    ivec2 texSize = textureSize(tex, 0);
    
    // Handle boundaries like MSA implementation
    if (pos.x <= 0) pos.x = 1;
    if (pos.x >= texSize.x - 1) pos.x = texSize.x - 2;
    if (pos.y <= 0) pos.y = 1;
    if (pos.y >= texSize.y - 1) pos.y = texSize.y - 2;
    
    return texelFetch(tex, pos, 0);
}

// Jacobi iteration for diffusion (similar to MSA linearSolver)
vec4 diffuse(ivec2 pos) {
    float a = u_deltaTime * u_viscosity * float(textureSize(u_fluid, 0).x * textureSize(u_fluid, 0).y);
    float c = 1.0 + 4.0 * a;
    
    vec4 center = sampleFluid(u_fluid, pos);
    vec4 left = sampleFluid(u_fluid, pos + ivec2(-1, 0));
    vec4 right = sampleFluid(u_fluid, pos + ivec2(1, 0));
    vec4 top = sampleFluid(u_fluid, pos + ivec2(0, -1));
    vec4 bottom = sampleFluid(u_fluid, pos + ivec2(0, 1));
    
    return (a * (left + right + top + bottom) + center) / c;
}

// Project step to enforce incompressibility (similar to MSA project)
vec2 project(ivec2 pos) {
    ivec2 texSize = textureSize(u_fluid, 0);
    float h = -0.5 / float(texSize.x);
    
    vec4 center = sampleFluid(u_fluid, pos);
    vec4 left = sampleFluid(u_fluid, pos + ivec2(-1, 0));
    vec4 right = sampleFluid(u_fluid, pos + ivec2(1, 0));
    vec4 top = sampleFluid(u_fluid, pos + ivec2(0, -1));
    vec4 bottom = sampleFluid(u_fluid, pos + ivec2(0, 1));
    
    // First compute divergence (like MSA)
    float divergence = h * (right.x - left.x + bottom.y - top.y);
    
    // Solve pressure (Gauss-Seidel relaxation)
    float pressure = 0.25 * (left.w + right.w + top.w + bottom.w - divergence);
    
    // Subtract gradient with proper scaling
    float fx = 0.5 * float(texSize.x);
    float fy = 0.5 * float(texSize.y);
    vec2 gradient = vec2(
        fx * (right.w - left.w),
        fy * (bottom.w - top.w)
    );
    
    return center.xy - gradient;
}

// Advection with proper boundary handling (similar to MSA advect)
vec4 advect(ivec2 pos) {
    ivec2 texSize = textureSize(u_fluid, 0);
    float dt0x = u_deltaTime * float(texSize.x);
    float dt0y = u_deltaTime * float(texSize.y);
    
    // Get current velocity at this position
    vec4 current = sampleFluid(u_fluid, pos);
    
    // Calculate previous position with proper scaling
    float x = float(pos.x) - dt0x * current.x;
    float y = float(pos.y) - dt0y * current.y;
    
    // Clamp to boundaries like MSA
    x = clamp(x, 0.5, float(texSize.x) + 0.5);
    y = clamp(y, 0.5, float(texSize.y) + 0.5);
    
    // Get integer coordinates
    int i0 = int(x);
    int j0 = int(y);
    int i1 = i0 + 1;
    int j1 = j0 + 1;
    
    // Get fractional parts
    float s1 = x - float(i0);
    float s0 = 1.0 - s1;
    float t1 = y - float(j0);
    float t0 = 1.0 - t1;
    
    // Sample using bilinear interpolation with proper boundary handling
    vec4 i0j0 = sampleFluid(u_fluid, ivec2(i0, j0));
    vec4 i1j0 = sampleFluid(u_fluid, ivec2(i1, j0));
    vec4 i0j1 = sampleFluid(u_fluid, ivec2(i0, j1));
    vec4 i1j1 = sampleFluid(u_fluid, ivec2(i1, j1));
    
    // Bilinear interpolation
    return s0 * (t0 * i0j0 + t1 * i0j1) + 
           s1 * (t0 * i1j0 + t1 * i1j1);
}

void main() {
    ivec2 pos = ivec2(gl_FragCoord.xy);
    ivec2 texSize = textureSize(u_fluid, 0);
    
    // Skip boundary cells
    if (pos.x <= 0 || pos.x >= texSize.x-1 || pos.y <= 0 || pos.y >= texSize.y-1) {
        fragColor = vec4(0.0);
        return;
    }
    
    vec4 state = sampleFluid(u_fluid, pos);
    vec2 vel = state.xy;    // velocity
    float density = state.z; // density
    float pressure = state.w; // pressure
    
    // Following MSA sequence:
    // 1. Add source (emitter)
    float emitterRadius = float(texSize.x) * 0.1;
    vec2 emitterPosPixels = u_emitterPos * vec2(texSize);
    float dist = distance(vec2(pos), emitterPosPixels);
    if (dist < emitterRadius) {
        float influence = smoothstep(emitterRadius, 0.0, dist);
        vel += u_emitterDir * u_emitterStrength * influence * 2.0;
        density += u_emitterDensity * influence * 1.5;
    }
    
    // 2. Diffuse velocity
    vec4 diffused = diffuse(pos);
    vel = diffused.xy;
    
    // 3. Project to enforce incompressibility
    vel = project(pos);
    
    // 4. Advect
    vec4 advected = advect(pos);
    vel = advected.xy;
    density = advected.z;
    
    // 5. Project again
    vel = project(pos);
    
    // Apply dissipation
    vel *= u_dissipation;
    density *= u_densityDissipation;
    
    // Zero out very small velocities to prevent denormal numbers
    const float ZERO_THRESH = 1e-9;
    if (abs(vel.x) < ZERO_THRESH) vel.x = 0.0;
    if (abs(vel.y) < ZERO_THRESH) vel.y = 0.0;
    
    // Clamp final values
    vel = clamp(vel, vec2(-1.0), vec2(1.0));
    density = clamp(density, 0.0, 1.0);
    
    fragColor = vec4(vel, density, pressure);
}
`;

const renderGlsl = glsl`
#version 300 es
precision highp float;

uniform sampler2D u_fluidR;
uniform sampler2D u_fluidG;
uniform sampler2D u_fluidB;
in vec2 vUv;
out vec4 fragColor;

void main() {
    // Sample density from each color channel
    float r = clamp(texture(u_fluidR, vUv).z, 0.0, 1.0);
    float g = clamp(texture(u_fluidG, vUv).z, 0.0, 1.0);
    float b = clamp(texture(u_fluidB, vUv).z, 0.0, 1.0);
    
    fragColor = vec4(r, g, b, 1.0);
}
`;
