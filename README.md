# Rust Invaders using the Bevy game engine

> <img src="assets/player_a_01.png" width=20 height=10> go easy that's a first try

1. [But why ?](#but-why-)
2. [Dependencies](#dependencies)
3. [Build](#build)
4. [Changelog](#changelog)
5. [Bugs and TODOs](#bugs-and-todos)

## But why ?

Don't you like crabs? Ok, this one shoots lasers. Still? Nevermind, that just a game, and you can change the appearance of your character anyway. This is some cheap space invaders clone so you move using the left and right arrows, and shoot using space.

## Dependencies

Ok so on a perfect world cargo would take care of that for you, but if for some reason you want some fancy fast-compiling features, here they are :

```toml
[dependencies]
bevy = "*"
rand = "*"
```

## Build

Compile the project with

```bash
cargo build #--release
```

Run the project with

```bash
cargo run #--release
```

## Changelog

1.  I hope Bevy is fast... why learning slow btw ?
2.  Time to find some pngs
3.  Ah plugins
4.  Explosion animation

## Bugs and TODOs

1.  Why is that that I can go outside of the window?