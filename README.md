# Cyberspace

*STILL A WIP, PoC of developing a Bevy game with Sui*

Cyberspace is an early attempt at a [meta](https://en.wikipedia.org/wiki/Metagame)/[autonomous world](https://0xparc.org/blog/autonomous-worlds) game set on the Sui Blockchain built using Bevy.

This game intends to gamify existing on-chain actions into playable quests that are familiar to strategy game enthusaists. The popular [Civilization](https://en.wikipedia.org/wiki/Civilization_(series)) game series has a set of winning conditions: Economic, Diplomatic, Cultural victory, etc. We map this these victory conditions into on-chain actions respectively: best PnL over a game duration, most varied set of move objects owned + unique addresses transacted, greatest collection rare NFTs, etc.

### Background / Architecture
The game is devloped using the [Bevy Engine](https://bevyengine.org/), a popular open source alternative to game development. The "backend" of the game is the chain itself. The game is built as a wasm target deployed on a web app.

Users can login to the web app using zkLogin, the React web app can communciate to the game using events passed through the wasm boundary.

--
#### Smart Contract
The Game program is found in `contracts`. A game initiator calls `create_game` to generate a random seed (map seed) and a player registry. Randomness uses on-chain randomness provided by the Random module. Users join the game by calling `enter_game` with the shared object.

#### Game
The game application is written using Bevy Engine which is an event-driven system. It targets wasm which makes it easy to deploy to the web. Rust's wasm-bindgen crate makes it possible to pass onchain state into the game (& vice-versa).

The game map uses on-chain randomness. The players and leaderboard are also using on-chain objects/state. See _further work_ for more.

Credits to https://kenney.nl/assets/medieval-rts for assets.

#### Web App
The app is a regular React app. It uses zkLogin to easily onboard/connect a user to the game. Credits to https://github.com/juzybits/polymedia-zklogin-demo for reference on implemention.

#### Further Work
- Users can place objects/settlements in the game based on their winning condition stats. These objects are registered with Sui allowing the Game to completely use Sui as a backend.
- Users can see other users in the game (requires WebRTC i think).
- Users can start guilds with each other in order to advance their spot in the leaderboards.

![Screenshot of game](/screenshot.png)

## Build
```sh
cargo install wasm-server-runner
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --typescript --target web     --out-dir ./out/     --out-name "cyberspace"     ./target/wasm32-unknown-unknown/release/cyberspace.wasm
python3 -m http.server --bind 127.0.0.1 8080
```
