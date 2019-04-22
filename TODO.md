# Todo

#### styles should support fill AND/OR stroke - not just OR

#### handle input events
we need to define what the input events will be.
it needs to be cross platform, so we need some abstractions that span platforms.

one interesting example is keyboard control for a desktop vs on-screen joystick control for a mobile app.  the on-screen joystick is not really part of the app, its part of the overlay that generates the input events for the app.  so how would we want to abstract across those?

or i guess another simple one could be key pushes from buttons from a controller (not a keyboard).  

and with keyboard, it works a little different on mobile vs desktop.

can we abstract these across platforms, neglecting certain platform specific gestures / inputs?
or do we need a platform specific way to convert inputs into state changes?
or something else?

any given app state needs to produce the possible actions (or functions to generate parameterized actions) - as well as information about how to invoke those actions.

##### input events as state transitions
perhaps it is more appropriate to say that any given app state needs to provide the list of possible state transitions (or functions to generate parameterized state transitions).  then we need some layer above that to map actual inputs to those state transitions.

it also seems hard to completely separate the app from the different platforms.  you'd have to know at least the constraints of how you want your app to be run.  e.g. if you need to be able to trigger 25+ possible actions quickly and easily, you're gonna need a keyboard or other high-button-count input device.  if you have a lot of data to display, you may need a minimum screen size.  i suppose we can bake those constraints into the app, and it can check the device or provide a warning or something.

the app itself is built on rainbow-unicorn, and various targets are built using rainbow-unicorn-{platform}, where each target has logic (as-needed) to map platform specific device inputs to some appropriate abstraction of rainbow-unicorn state transitions or inputs?

#### get the whole input -> state update -> primitives -> paint pipeline working
tick -> process input -> update -> render -> paint

tick - triggers the pipeline, with the latest timestamp
process input - converts the input queue into state transitions
update - process state transitions to produce the next state
render - convert the state to graphics primitives
paint - draw the graphics primitives to the device

#### implement transforms on primitives

#### add image or bitmap or whatever as primitive

#### add more timeline functions

#### add object abstractions?  e.g. eye, leg, person, etc...
although maybe that is not part of core?  they are just structs along with functions that transform them into primitives.  i feel thats an 'app', or set of utilities used by rainbow-unicorn apps, but not inherently a part of rainbow unicorn.