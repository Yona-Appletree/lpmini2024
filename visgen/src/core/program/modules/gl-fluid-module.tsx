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

// Helper function to get fluid cell index
ivec2 fluidIX(ivec2 pos) {
    return clamp(pos, ivec2(0), textureSize(u_fluid, 0) - 1);
}

// Improved bilinear sampling with boundary handling
vec4 sampleBilinear(sampler2D tex, vec2 uv) {
    vec2 texSize = vec2(textureSize(tex, 0));
    vec2 texelSize = 1.0 / texSize;
    
    // Clamp to edge
    uv = clamp(uv, texelSize, 1.0 - texelSize);
    
    vec2 texelCoord = uv * texSize - 0.5;
    vec2 f = fract(texelCoord);
    texelCoord = (floor(texelCoord) + 0.5) / texSize;
    
    vec4 tl = texture(tex, texelCoord);
    vec4 tr = texture(tex, texelCoord + vec2(texelSize.x, 0.0));
    vec4 bl = texture(tex, texelCoord + vec2(0.0, texelSize.y));
    vec4 br = texture(tex, texelCoord + vec2(texelSize.x, texelSize.y));
    
    return mix(
        mix(tl, tr, f.x),
        mix(bl, br, f.x),
        f.y
    );
}

// Improved advection based on MSA Fluid
vec4 advect(vec2 pos, vec2 vel) {
    vec2 prevPos = pos - vel * u_deltaTime;
    return sampleBilinear(u_fluid, prevPos);
}

// Jacobi iteration for diffusion
vec4 diffuse(vec2 pos) {
    ivec2 texSize = textureSize(u_fluid, 0);
    vec2 texelSize = 1.0 / vec2(texSize);
    
    vec4 center = texture(u_fluid, pos);
    vec4 left = texture(u_fluid, pos + vec2(-texelSize.x, 0.0));
    vec4 right = texture(u_fluid, pos + vec2(texelSize.x, 0.0));
    vec4 top = texture(u_fluid, pos + vec2(0.0, texelSize.y));
    vec4 bottom = texture(u_fluid, pos + vec2(0.0, -texelSize.y));
    
    float alpha = texelSize.x * texelSize.x / (u_viscosity * u_deltaTime + 1e-6);
    float rBeta = 1.0 / (4.0 + alpha);
    
    return (left + right + top + bottom + alpha * center) * rBeta;
}

// Project step to enforce incompressibility
vec2 project(vec2 pos) {
    ivec2 texSize = textureSize(u_fluid, 0);
    vec2 texelSize = 1.0 / vec2(texSize);
    
    vec4 center = texture(u_fluid, pos);
    vec4 left = texture(u_fluid, pos + vec2(-texelSize.x, 0.0));
    vec4 right = texture(u_fluid, pos + vec2(texelSize.x, 0.0));
    vec4 top = texture(u_fluid, pos + vec2(0.0, texelSize.y));
    vec4 bottom = texture(u_fluid, pos + vec2(0.0, -texelSize.y));
    
    float divergence = -0.5 * (
        right.x - left.x +
        top.y - bottom.y
    ) / texelSize.x;
    
    float pressure = (left.w + right.w + top.w + bottom.w - divergence) * 0.25;
    
    vec2 gradient = 0.5 * vec2(
        right.w - left.w,
        top.w - bottom.w
    ) / texelSize.x;
    
    return center.xy - gradient;
}

void main() {
    vec2 pos = gl_FragCoord.xy / vec2(textureSize(u_fluid, 0));
    vec4 state = texture(u_fluid, pos);
    
    // Extract current state
    vec2 vel = state.xy;    // velocity
    float density = state.z; // density
    float pressure = state.w; // pressure
    
    // Apply diffusion
    vec4 diffused = diffuse(pos);
    vel = diffused.xy;
    density = diffused.z;
    
    // Apply advection
    vec4 advected = advect(pos, vel);
    vel = advected.xy;
    density = advected.z;
    
    // Project to enforce incompressibility
    vel = project(pos);
    
    // Apply dissipation
    vel *= u_dissipation;
    density *= u_densityDissipation;
    
    // Add emitter influence
    float emitterRadius = 0.1;
    float dist = distance(pos, u_emitterPos);
    if (dist < emitterRadius) {
        float influence = smoothstep(emitterRadius, 0.0, dist);
        vel += u_emitterDir * u_emitterStrength * influence * 2.0;
        density += u_emitterDensity * influence * 1.5;
    }
    
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
