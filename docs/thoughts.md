# LightPlayer design

I want to come up with an overall architecture for the lightplayer engine.
Focusing on the current MVP goals, but keeping in mind larger goals.

## System design

- LightPlayer is a scalable system designed to display interactive animations on addressable leds.
- Usability and ease of use are key design goals.
- Client/server architecture with a firmware module and a web ui module.
- Communication between firmware and ui is done through a single pipe, such as serial
  over usb, or bluetooth. In some cases we may have a wifi connection.
- 2d visualizations that are mapped on to pixels
-

## Current state

- FxPipeline with its own buffers and steps that are executed
- fixed mapping
- lpscript language for expressions and scripts
- stubs for frontend code

## MVP goals

- UI
    - Accessible to mobile users, scales up for desktop
    - Ideally just web-based for now, but due to communication limitations, might have to make a
      mobile app for access to serial, bluetooth
- Targets
    - ESP32c3 is the main production target
    - WebAssembly so the UI can be tested without actual hardware
- Scale
    - 50-250 RGB pixels
    - 16x16 2d visualization
- Editing
    - We need live editing, where the user can in real time make changes to the scene and see it in
      real time
- Communication
    - The firmware runs on esp32, the ui in a browser, and we use WebSerial to talk to the device
    - Bluetooth is a stretch goal, but I don't think mobile browsers support it
- Supports customizable inputs
    - must have: buttons, potentiometers
    - bonus: sound, rotary encoders
- One WS2811 output
    - Must be configurable pin
    - Supports "scanning" mode where we iterate through the pins to help the user figure out which
      pin the strip is attached to
- Visualizations
    - Are some kind of pipeline of composable stages, some of which may be shader-based.
    - Must have the ability to define multiple visualizations that can be switched between.
- Debugging
    - Debugging is a key feature. We must gracefully handle errors and crashes of the firmware.

## Future goals

- Scale to run on full machines with GPUs where GLSL is executed natively

## Current architecture thoughts

Whenever I think through this, I end up imagining a node-based programming system of some kind,
where data is "pulled" by some **output**/terminal node from other nodes.

Perhaps the ui reflects that, perhaps not. node-based uis are somewhat cumbersome.

It's important to me to allow for reusable patterns so that people can share them with each other.

I imagine a set of "entities" or perhaps "modules" that are configured and wired together, something
like this:

Example scene in example-scene.toml

## Questions

For now, I want to focus on figuring out the core data structures to allow something like the above.

To help me:
Look at lpscript, engine-core, lp-debug
Ignore lpcore for now, it was an old idea.

Just read the above, ant don't make a full plan yet, I want to work interactively about this idea.
Mostly I want to iterate on the above example scene to work out a good model.

## Answer

1. in toml that's just syntax for a property, so its perlin: { children: { lfo : { type: "math:
   lfo" }}}
   and yes, its a child entity

