import { TextureDef } from "@/core/data/types/texture-def.tsx";
import { FloatDef } from "@/core/data/types/float-def.tsx";
import { RecordDef } from "@/core/data/types/record-def.tsx";
import { glsl } from "@/frontend/util/glsl.ts";
import { defineModule } from "@/core/program/define-module.ts";
import { Gl2dFragmentShader } from "@/core/gl2d/gl2d-fragment-shader.ts";
import type { RuntimeContext } from "@/core/program/program-runtime.ts";
import { useEffect, useRef } from "react";
import { Vec2Def } from "@/core/data/types/vec2-def.tsx";

export const GlFluidModule = defineModule(
  "gl-fluid",
  {
    label: "Fluid Simulation",
    input: RecordDef({
      viscosity: FloatDef({ default: 0.0001 }),
      diffusion: FloatDef({ default: 0.00001 }),
      velocityDissipation: FloatDef({ default: 0.99 }),
      densityDissipation: FloatDef({ default: 0.99 }),
      deltaTime: FloatDef({ default: 0.016 }),
      emitterLocation: Vec2Def({ default: [0.5, 0.5] }),
      emitterDirection: Vec2Def({ default: [0.0, 1.0] }),
      emitterStrength: FloatDef({ default: 0.3 }),
      emitterDensity: FloatDef({ default: 1.0 }),
    }),
    output: TextureDef(),
  },
  ({ gl2d }) => {
    // Create shader for combined fluid simulation
    const fluidShader = Gl2dFragmentShader(gl2d.context, fluidStepGlsl.trim());

    // Create shader for rendering
    const renderShader = Gl2dFragmentShader(gl2d.context, renderGlsl.trim());

    // Create framebuffers for double buffering simulation state
    const fluidBuffer1 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });
    const fluidBuffer2 = gl2d.framebuffer({
      options: { format: "float32", filter: "nearest" },
    });

    // Create framebuffer for rendering
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

    return {
      update: ({ input }) => {
        // Swap buffers
        const readBuffer = frame % 2 === 0 ? fluidBuffer1 : fluidBuffer2;
        const writeBuffer = frame % 2 === 0 ? fluidBuffer2 : fluidBuffer1;

        // Update fluid state
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
          u_emitterDensity: { type: "float32", value: input.emitterDensity },
        });

        // Render the density visualization
        renderBuffer.bind();
        renderShader.draw({
          u_fluid: { type: "texture", value: writeBuffer.texture },
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

void main() {
    vec2 coord = gl_FragCoord.xy / vec2(textureSize(u_fluid, 0));
    
    // Sample current state
    vec4 state = texture(u_fluid, coord);
    vec2 v = state.xy;    // velocity
    float d = state.z;    // density
    float p = state.w;    // pressure
    
    // Sample neighbors for Laplacian
    vec4 left = texture(u_fluid, coord + vec2(-1.0, 0.0) / vec2(textureSize(u_fluid, 0)));
    vec4 right = texture(u_fluid, coord + vec2(1.0, 0.0) / vec2(textureSize(u_fluid, 0)));
    vec4 top = texture(u_fluid, coord + vec2(0.0, 1.0) / vec2(textureSize(u_fluid, 0)));
    vec4 bottom = texture(u_fluid, coord + vec2(0.0, -1.0) / vec2(textureSize(u_fluid, 0)));
    
    // Update velocity
    vec2 vLaplacian = left.xy + right.xy + top.xy + bottom.xy - 4.0 * v;
    v += u_viscosity * vLaplacian * u_deltaTime;
    v *= u_dissipation;
    
    // Update density
    float dLaplacian = left.z + right.z + top.z + bottom.z - 4.0 * d;
    d += u_diffusion * dLaplacian * u_deltaTime;
    
    // Advect density
    vec2 prevCoord = coord - v * u_deltaTime;
    d = texture(u_fluid, prevCoord).z;
    d *= u_densityDissipation;
    
    // Update pressure
    float divergence = 0.5 * (
        right.x - left.x +
        top.y - bottom.y
    );
    p = (left.w + right.w + top.w + bottom.w - divergence) * 0.25;
    
    // Apply pressure force to velocity
    vec2 pressureGradient = vec2(
        right.w - left.w,
        top.w - bottom.w
    ) * 0.5;
    v -= pressureGradient;
    
    // Add emitter
    float emitterRadius = 0.05;
    float dist = distance(coord, u_emitterPos);
    if (dist < emitterRadius) {
        float influence = (1.0 - dist / emitterRadius);
        v += u_emitterDir * u_emitterStrength * influence;
        d += u_emitterDensity * influence;
    }
    
    fragColor = vec4(v, d, p);
}
`;

const renderGlsl = glsl`
#version 300 es
precision highp float;

uniform sampler2D u_fluid;
in vec2 vUv;
out vec4 fragColor;

void main() {
    vec4 fluid = texture(u_fluid, vUv);
    float density = fluid.z; // density is stored in blue channel
    fragColor = vec4(density, 0.0, 0.0, 1.0); // output as red
}
`;
