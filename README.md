# opengl_graphics [![Build Status](https://travis-ci.org/PistonDevelopers/opengl_graphics.svg)](https://travis-ci.org/PistonDevelopers/opengl_graphics)

An OpenGL back-end for Rust-Graphics

Maintainers: @TyOverby, @bvssvni, @Coeuvre

### Important!

OpenGL needs to load function pointers before use.
If you are experiencing strange error messages like "X not loaded" this is likely the case.
This is done automatically for you in the SDL2 and GLFW window back-ends for Piston.
To do this manually, see the README in [gl-rs](https://github.com/bjz/gl-rs)
