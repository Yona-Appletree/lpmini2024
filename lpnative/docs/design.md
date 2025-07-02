# Enities

Lightplayer programs are defined by a hierarchy of **entities** each of which have **inputs** and
**outputs** that can be wired together to form a directed acyclic graph.

An entity is defined by:

- **input** - data provided to the entity from configuration or other entities
- **state** - private data the entity holds between frames
- **output** - data provided by the entity to other entities
- **logic** - code that is run once per frame if the entity needs to be updated

There are three kinds of entities:

- **Output** entities affect the outside world, such as controlling LEDs
- **Input** entities bring in data from the outside world, such as a camera or microphone
- **Processing** entities transform input to be used by other entities

Entities may have internal state available from frame to frame. State must be serializable to
allow synchronization between instances of a program.

# Dynval

Dynnamic values, `dynval` for short, are the core of lightplayers data system,
allowing entities to define their inputs, state, and output, and wire those
value together.

They are essentially a reactive value system, like
