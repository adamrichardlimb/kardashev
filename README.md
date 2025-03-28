# Kardashev

Kardashev is a (yet another) sandbox voxel game based, this time based around technological progression and physics.

## TODO

- [ ] Move all SDL2 features into an SDL2 struct which exposes what we need and abstracts away the details.
- [ ] Create Renderable trait and make it easy for Renderable objects to add themselves to the queue.
- [ ] Start implementing benchmarking tools, tests.
- [ ] Optimise texture drawing so old textures are discarded so IDs can be re-used.
- [ ] Optimise meshes so we have one mesh per chunk and not one mesh per voxel.
- [ ] Introduce physics and new controllers.
- [ ] Basic GUI stuff
- [ ] Add a menu now that you have text rendering.
- [ ] Begin work on world generation based on planets.
- [ ] Once all of the above is done, add networking so players can send seeds to one another and walk around on small sample planets.
