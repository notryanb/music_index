# Music Index

Experimenting with ways to search my music library in Rust.

## State of the project
- Searches for a single term that occurs within the artist name
- Builds a database with a json list and some ids (ulid-lite)
- indexes the database with a pseudo-inverted index in memory (this is build every time.... blargh)
- uses the index to find mp3 tag records in the database.
- All logic is in one terrible function.

## How to use
- Clone the repo
- first arg is search path (aka where to build db from if it doesn't exist) and second arg is the search term.
- `cargo run --release -- 'E:/Mp3' fugazi`

### Things to explore
- Inverted index
- support multiple terms
- support term across multiple fields (artist, album, track title)
- database format (json, bson, etc...)
- store multiple search paths
- Reduce String allocation
- Fuzzy / better word searching.
- async / concurrent database building
- concurrent access to datase / query planning.