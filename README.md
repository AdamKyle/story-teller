# Story Teller.

Story teller is a game that I developer in a modular way in rust. While it is a
rudimentary text based adventure game, the way we do things is through modules.

The only core rule here is that `core/` can not depend on any other module and
must be clean.

## Getting Started:

- clone the project
- `cargo build`
- `cargo run`

## Why?

I wanted to learn rust and I was also inspired by Halt and Catch Fire to create
an old school text adventure game in a new language that was "close" to the metal
and not C. Something a little higher then C.

### Is there a Story?

Story teller is a collection of stories that all tell a story I have been writing called: The Child and The Poet.
Each game in the collection will have its own story, for example here is the current itteration of the entry for Dark Harvest:

```
==== [Dark Harvest] ===

Introduction:

    Welcome to Dark Harvest. This is the first chapter in the story of: The Child and The Poet, a dark story
    revolving around love, loss and other worlds. While inspired by DND, Dark Harvest doesn't follow any rule books
    closely. Only as a reference.

Story:

You awaken in the middle of a field. Covered in blood, you have no idea how you got here. As you stand and adjust your eyes to the blinding sun, you feel your body for wounds, to find the source of the bleeding. No wounds present them selves.
The wind kicks up and a voice is heard, like a whisper moving through the shadows, "Who are you?"


Adventure Difficulty: Easy
Adventure Length:      Medium
=======================
```

## Modules

- `core/` - Contains all the core logic of the game. Used across multiple modules.
- `game/` - Contains all the core game logic from executing actions and moving around rooms.
- `character/` - Contains all the logic for a character.
- `adventure/` - Contains the modules of each adventure in this game.
- `src/` - The actual game.

## Actions in Game:

Actions are done through nouns. For example: `walk north`.

- Upper case and capitalization's are converted to lowercase.

| Action   |     what you type      |  Accepted Arguments | Example |
|:----------|:-------------:|:------:|:---:|
| Look (when in an adventure) |  look | N/A | `> look` |
| Movement (when in an adventure) |    go/walk  |  n(orth), s(outh),e(ast),w(est), back | `> walk north`, `> go s`, `>go back` |
| Explore (when in an adventure) | explore |    N/A | `> explore` |
| Talk (when in adventure) | converse, talk | N/A | `> talk` or `> converse` |
| Quit  (at any time)| q, quit, exit| N/A | type: `quit` or `q` or `exit` |
| Accept | accept | N/A | `> accept` |
| Re-roll (only in character stat creation) | re-roll | N/A | `> re-roll` |
| Help (when in an adventure) | help | N/A | `> help` |

## Current Adventures:

Currently the game contains one adventure: Dark Harvest.

### Adventures:

All adventures in the game are made of a world object that then contains rooms in a top down fashion
where the exits allow the player to branch off into different rooms.

Currently a player can go back to the previous room, assuming that the room allows them to head back.
