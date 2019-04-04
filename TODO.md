# Todo

#### move html canvas stuff out to a 'rainbow-unicorn-wasm' module.
that module would have the web-specific drawing logic.  it can also have example usage.

We'll need to make an abstraction for a drawing context.
Then each `rainbow-unicorn-{platform}` module will implement that drawing context for its platform.  We can make one for android, swing, whatever.

Then we can test `rainbow-unicorn` with a fake drawing context.

#### create the state to primitives function in 'rainbow-unicorn'.
We need to make the generic state-to-primitives function part of the framework.


#### move animation logic into 'rainbow-unicorn'
the animation stuff should be the same regardless of the platform we're running on.  also, it isn't 'animation' - its time-based property values.  Any property could conceivably be a function of time, not just the shapes.  i.e. If we have an app with an abstract data model that has nothing to do with the shapes, it can still have properties that are a function of time that may or may not result in visual changes.
