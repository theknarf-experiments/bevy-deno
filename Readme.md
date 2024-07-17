# Bevy Deno

Run JavaScript and TypeScript as a plugin in Bevy.
Can be used to build game UI's, game logic, reusable plugins or to support end-user modding.

# (Planned) Features

- Define a script from code

- Define a script from file

  - Work with Vite

    - Support hot-reloading of file-changes

    - Allow you to bundle everything together and include_str! it in code

    - Maybe expose a custom macro? Something like vite_bunlde! or something?

- Support both JS and TS

- Support loading modules from `node_modules`

- Expose functions for adding, removing & updating Bevy components

- Work with React, custom react runner / reconciler

  - Use React to define components, systems, etc

  - Use it either to just build UI's, or the entire app / game

- Expose library functions to allow other libraries to build on top of it

  - That way a Bevy plugin could use this to build a small UI in React, and the consumer of that plugin wouldn't need to know / setup anything special
