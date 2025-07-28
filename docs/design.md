# Lightplayer

Lightplayer is a node-based programming system for the purpose of building
interactive LED art.

The goal is to provide LED artists an environment to build the software
component of their work with a simple but powerful interface.

It consists of a server component that runs headlessly on a device and
an interactive web interface for designing the visualizations.

# Scene

A scene is the top-level concept in Lightplayer, and provides the main
context for execution and user interface.

Scenes consist of a graph of entities, where the inputs of entites are
based on the outputs of others.

Scenes execute in steps called frames. In each frame, all entites which
produce side-effcets are **updated**. The upstream entities connected
to the inputs of effect entities are updated first.

Some entity inputs may be designated as "lazy" which allows for partial
execution of the graph. This enables "switch" or "selection" entities
which can control which parts of the program are active at any particular
moment.

# Enities

Entities are the small programs that make up a scene. An instance of an entity
in a scene is called a node.

# Node

A node is an instance of an entity in a scene.

## Input

Entities specify the data they need to produce their output or effect. This data
may be constant data specified by the user, or dynmaic data derived from other
entities.

The shape of entity input is used to dynamically generate the user interface,
and is a core concept.

Some inputs can be designated as "lazy" meaning that the value may not be
needed in a particular update.

## Output

The output of an entity is what can be used by other entites as inputs.

The user can also view the outputs of entities, which can be usefun when
debugging.

## State

Entities can have internal state that they can use to track data between
frames.

The output and effect of an entity must be computable from the combination
of the input and state of the entity.

State must be serializable so that multiple instances of lightplayer can
be synchonized.

## External state

Entities can be designated as depending on external state, such as a
physical input.

These entites have an extra function used to update their state on
any frame where their data is needed.

## Effects

Entities can be designated as producing an **effect** based on their input,
indicating that they don't just output a value, but directly affect
the real world.

Entities with effects are considered the leaf nodes of the entity graph,
and are always updated on every frame.

# Module

A module is an entity composed of other entites, allowing reuse and organziation
in complex programs.
