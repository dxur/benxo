# Todo list

## High priority
[ ] Add Other page to the frontend.
[ ] Site theme based generation. !important
[x] Rewrite the dahboard using svelte.
[x] Merge the common and backend crates.
[x] Fix db schema.
[x] Enhance the Product page to the frontend.
[x] Add Orders page to the frontend.
[x] Fix the add new variant.

## Low priority
[x] Auto generate form using proc-macros
[x] Fix the `routes` macro to implement the `Into<Router>` for `ApiRoutes`

## Draft
I'm conserning using a schema based frontend implementation for more DRY code and declarative
approach this will help scalling the app and maybe auto generating the frontend frome the backend
directly using macros and schema. and may help serving region based schema/dynamic schemas directly
from the backend.
Basic schema prototype
```json
{
   
}
```

Conserning the frontend implementation I'm considering a runtime solution for ease of development
and/or dyncamic schemas as said earlier. My basic idea for schema implementation is to have the
schema as a parameter for the page/ui builder/component schemas should be flexible UI ignostic
for less coupled system. Iterations on the implementation may enhance the overall idea.
