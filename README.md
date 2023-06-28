# Match 3 with Bevy

A digital birdhouse by Joel Shinness

- [X] Ability to see a grid of 6x6 tiles
- [X] Derive 6x6 tiles from a Grid vec
- [X] Add images (probably from https://kenney.nl/assets/puzzle-pack-2)
- [ ] Add swapping adjacent tiles with touch
- [ ] Shape recognition / Scoring
- [ ] Remove tiles and let new ones drop
- [ ] Add animations for the tiles removing and dropping
- [ ] Add menus / initial screen / sounds / settings (make it a whole app)

## What is ECS?

ECS is a game architecture framework that has 3 big sections:

- Entities: (ID's to which we attach Components)
- Components: (Which represent data and queryable aspects of things in our game)
- Systems: (functions that query and act on Components and Entities in the game)

## What are some things in our game?

### Entities

* Blocks
* Grid

### Components

* Block
    * Type
    * Location
* LifeTime
* Dropping
  * Offset
  * Time Remaining

## Systems

* `setup_grid`
* `is_moving`
* `remove_expired_tiles`
* `drop_tiles`
