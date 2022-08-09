# Rusty Duke

Unofficial, non-commercial and open source clone of the [The Duke](https://boardgamegeek.com/boardgame/257601/duke-lords-legacy) board game. Currently, only supports the basic tile set.

See the official game for rules.

The AI is decent, but feel free to contribute if you want more of a challenge. The heuristics are quite naive, so there should be some low-hanging fruits to pick.

I started this project because I wanted to learn Rust and do some AI development. I also want to be able to play 'The Duke' online with friends.

## Try it

### 2D Graphics (Work in progress)

Front-end implemented with [Bevy](https://bevyengine.org/).

### Terminal

Supports player vs. AI and AI vs. AI. Mainly for early manual testing of the game logic and AI. Will probably not get updated. Front-end implemented with [Crossterm](https://docs.rs/crossterm/latest/crossterm/).

`cargo run -p rusty-duke-terminal --release`

## Roadmap

1. 2D graphics with Bevy. (WIP)
2. Github CI/CD
3. Multiplayer support with Nakama.
4. Enhanced AI (I want to experiment with reinforcement learning)

## Credits
* Icons under CC0 1.0 Universal, by [Kenney Vleugels](https://www.kenney.nl).
