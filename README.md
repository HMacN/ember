# The Ember Game Library

## Dependencies
* [winit](https://github.com/rust-windowing/winit)

## Background
In my 3rd year at University I was given a sample game engine, which my lecturer had written himself using the Java language, and told to create a platformer game. I got a reasonable grade for the assignment, and quite enjoyed working on the project. I decided to try and replicate the engine in Rust, and to make a few (maybe not so small) improvements along the way. This project is done mainly as a learning activity, and is unlikely to end up with a particularly high-performance game engine. 

If I ever get it ~~finished~~ to a point where it can be used then I'll try to put together a few simple games to showcase it.  Any commentary, suggested improvements, desired features (or, someday, a link to a game someone builds with it!) are always very welcome.

## What this project is:
* A way for me to learn the Rust language.
* A way for me to practice using recognised Design Patterns.
* A way for me to practice Test-Driven Development (TDD).
* "Clean Code" as per the book by Robert C. Martin (ol' Uncle Bob himself) throughout, or at least that's the hope.
* A 2D game engine which is simple to use, and contains the minimum functions needed to make (very) basic games.
* As self-contained as possible.  It should include a minimum of dependencies, and zero game-specific dependencies (so I can use winit, but not bevy).

## What this project is not:
* 3D.
* Performant.
* Supported.
