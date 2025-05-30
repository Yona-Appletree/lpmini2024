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
      viscosity: FloatDef({ default: 0.001 }),
      diffusion: FloatDef({ default: 0.0001 }),
      velocityDissipation: FloatDef({ default: 0.995 }),
      densityDissipation: FloatDef({ default: 0.995 }),
      deltaTime: FloatDef({ default: 0.016 }),
      emitterLocation: Vec2Def({ default: [0.5, 0.5] }),
      emitterDirection: Vec2Def({ default: [0.0, 1.0] }),
      emitterStrength: FloatDef({ default: 0.1 }),
      emitterColor: Vec3Def({ default: [1.0, 0.5, 0.2] }), // RGB color
    }),
    output: TextureDef(),
  },
  ({ gl2d }) => {
    // Create shader for combined fluid simulation
    const fluidShader = Gl2dFragmentShader(gl2d.context, fluidStepGlsl.trim());

    // Create shader for rendering
    const renderShader = Gl2dFragmentShader(gl2d.context, renderGlsl.trim());

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

const fluidStepGlsl = glsl`
#version 300 es
precision highp float;

// Fluid state texture:
// R,G: velocity (x,y)
// B: density
// A: pressure
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

vec4 sampleBilinear(sampler2D tex, vec2 uv) {
    uv = clamp(uv, 0.0, 1.0); // Ensure we don't sample outside
    vec2 texSize = vec2(textureSize(tex, 0));
    vec2 texelSize = 1.0 / texSize;
    
    vec2 texelCoord = uv * texSize - 0.5;
    vec2 f = fract(texelCoord);
    texelCoord = (floor(texelCoord) + 0.5) / texSize;
    
    vec4 tl = texture(tex, texelCoord + texelSize * vec2(0.0, 0.0));
    vec4 tr = texture(tex, texelCoord + texelSize * vec2(1.0, 0.0));
    vec4 bl = texture(tex, texelCoord + texelSize * vec2(0.0, 1.0));
    vec4 br = texture(tex, texelCoord + texelSize * vec2(1.0, 1.0));
    
    return mix(
        mix(tl, tr, f.x),
        mix(bl, br, f.x),
        f.y
    );
}

void main() {
    vec2 coord = gl_FragCoord.xy / vec2(textureSize(u_fluid, 0));
    vec2 texelSize = 1.0 / vec2(textureSize(u_fluid, 0));
    
    // Sample current state
    vec4 state = texture(u_fluid, coord);
    vec2 v = state.xy;    // velocity
    float d = state.z;    // density
    float p = state.w;    // pressure
    
    // Sample neighbors for Jacobi iteration
    vec4 left = texture(u_fluid, coord + vec2(-texelSize.x, 0.0));
    vec4 right = texture(u_fluid, coord + vec2(texelSize.x, 0.0));
    vec4 top = texture(u_fluid, coord + vec2(0.0, texelSize.y));
    vec4 bottom = texture(u_fluid, coord + vec2(0.0, -texelSize.y));
    
    // Jacobi iteration for velocity diffusion (scaled down)
    vec2 vLaplacian = left.xy + right.xy + top.xy + bottom.xy;
    float alpha = 0.1 * texelSize.x * texelSize.x / (u_viscosity * u_deltaTime + 1e-6);
    float rBeta = 1.0 / (4.0 + alpha);
    v = (vLaplacian + alpha * v) * rBeta;
    
    // Apply velocity dissipation
    v *= max(1.0 - u_deltaTime, u_dissipation);
    
    // Semi-Lagrangian advection for velocity
    vec2 prevCoord = coord - v * u_deltaTime;
    v = sampleBilinear(u_fluid, prevCoord).xy;
    
    // Clamp velocity to prevent explosion
    v = clamp(v, vec2(-1.0), vec2(1.0));
    
    // Jacobi iteration for density diffusion (scaled down)
    float dLaplacian = left.z + right.z + top.z + bottom.z;
    alpha = 0.1 * texelSize.x * texelSize.x / (u_diffusion * u_deltaTime + 1e-6);
    rBeta = 1.0 / (4.0 + alpha);
    d = (dLaplacian + alpha * d) * rBeta;
    
    // Semi-Lagrangian advection for density
    prevCoord = coord - v * u_deltaTime;
    d = sampleBilinear(u_fluid, prevCoord).z;
    
    // Apply density dissipation
    d *= max(1.0 - u_deltaTime, u_densityDissipation);
    
    // Clamp density
    d = clamp(d, 0.0, 1.0);
    
    // Compute divergence (scaled down)
    float divergence = 0.1 * (
        right.x - left.x +
        top.y - bottom.y
    ) / texelSize.x;
    
    // Jacobi iteration for pressure
    float pLaplacian = left.w + right.w + top.w + bottom.w;
    p = (pLaplacian - divergence) * 0.25;
    
    // Clamp pressure
    p = clamp(p, -1.0, 1.0);
    
    // Apply pressure gradient to velocity (scaled down)
    vec2 pressureGradient = 0.1 * vec2(
        right.w - left.w,
        top.w - bottom.w
    ) / texelSize.x;
    v -= pressureGradient;
    
    // Add emitter with controlled strength
    float emitterRadius = 0.05;
    float dist = distance(coord, u_emitterPos);
    if (dist < emitterRadius) {
        float influence = smoothstep(emitterRadius, 0.0, dist);
        v += u_emitterDir * min(u_emitterStrength, 0.5) * influence;
        d += min(u_emitterDensity, 1.0) * influence;
    }
    
    // Final value clamping
    v = clamp(v, vec2(-1.0), vec2(1.0));
    d = clamp(d, 0.0, 1.0);
    p = clamp(p, -1.0, 1.0);
    
    fragColor = vec4(v, d, p);
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
