# Rusty Roguelike
Roguelike following the book "Hands-on Rust".

## Short description
A dungeon crawler with procedurally generated levels, monsters of increasing difficulty and turn-based movement.

## Story
The hero's hometown is suffering from a plague of monsters. Welling up from the deep, they seem unstoppable. Legend
tells of the Amulet of Yala that can be used to stem the tide. After a long night at the tavern, the hero promises to
save the day - and set forth into the dungeon.

## Basic game loops
1. Enter dungeon level
2. Explore, revealing the map
3. Encounter ennemies from whom the players decide to fight or flee from
4. Find power-ups and use them to strenghten the player
5. Locate the exit to the level, back to 1

## MVP
1. Create a basic dungeon map
2. Place the player and let them walk around
3. Spawn monsters, draw them, and let the player kill them by walking into them
4. Add health and a combat system that uses it
5. Add healing potions
6. Display a "Game Over" screen when the player dies
7. Add the Amulet of Yala to the level and let the player wins by reaching it

## Stretch goals
1. Add field of view
2. Add more interesting dungeon designs
3. Add some dungeon themes
4. Add multiple layers to the dungeon, with the amulet on the last one
5. Add varied weapons to the game
6. Move to a data driven design for spawning ennemies
7. Consider some visuals effects to make combats more visceral
8. Consider keeping scores
