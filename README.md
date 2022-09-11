# Saperke

An minesweeper game (Windows 98 style) written completely in Rust with a usage of [Druid UI toolkit](https://github.com/linebender/druid).

 ![Screenshot of the game with all subwindows](/assets/presentation/saperke-presentation.png)
 
## Features

- Standard predefined game difficulties
- Three predefined, unusual shapes 
- Possibility to set custom box-shaped game size
- Custom amount of bombs
- More precise game timer 
- Pause option
- The fact it works properly without major errors is a big feature

## Run it from source code

Download and run it as standard Rust project with `Cargo`:

```sh
cargo run
```

## ~~Not gonna lie, I will probably not add anything more~~ Todo/Nice to have:

- [ ] Fully functional map editor
- [ ] Better SVG parsing logic
- [ ] Leaderboard
- [ ] Possibility to run it in web (usvg doesn't compile for WASM at the time of this writing)

## Contributing

LoL, you nuts? 

Jokes aside - if you see some bugs (or ideas to add/improve something) I am open and will be very happy for PR's