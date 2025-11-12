# Rusty RYM Genre Hierarchies

This will be a simple tool to re-create the RateYourMusic (RYM) genre hierarchies in a
useful rust-accessible way.

## Usage

- You will need to provide the load function the data from this file: https://github.com/FlakyBlueJay/mb-rym-hierarchy/blob/main/RateYourMusic%20Genre%20Hierarchy.txt

- Once loaded... you can't do anything yet... once I write some more stuff you'll be able to do some stuff.

## Testing

- Clone this repo

- Download the list from [here](https://github.com/FlakyBlueJay/mb-rym-hierarchy) and put it in the data folder:

```
mkdir test
cd test
wget https://github.com/FlakyBlueJay/mb-rym-hierarchy/blob/main/RateYourMusic%20Genre%20Hierarchy.txt
```

- run test

```
cargo test
```

## Usage

TODO

## TODO

- Change the data structure to be more usable
- Add a way to search
- Add a way to search effectively (the only way I can think of right now is to iterate through everything.
  will probably require a change in data structure)
- Make this into a library

## Why?

Just for fun and my own education. I want to keep practicing Rust and try to learn more.

Maaaaybe, 4 years into the future, RYM might allow API access to their data and then I can
consider tagging my music using this and it should enable me to do broader tagging. For example, if
I have a Djent album, I probably also want it to come up when I search for "Metal" music. If the
song/album is only tagged "Djent", then it won't come up when searching "Metal", so adding the parent
hierarchical genre in there could be useful (maybe - it might also completely make things annoying
when a million albums come up when searching some things).
