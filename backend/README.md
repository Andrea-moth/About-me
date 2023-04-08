## Frontend 

This is where all the databases go

## Building

Make sure that there is a file named projects.db in both the root directory of the project and the backend folder[^1] 

This projects.db should have three tables

1. projects(name TEXT PRIMARY KEY NOT NULL UNIQUE, image_path TEXT NOT NULL UNIQUE, description TEXT NOT NULL UNIQUE)
2. charities(flavor_text TEXT PRIMARY KEY NOT NULL, link_text TEXT NOT NULL, link TEXT NOT NULL UNIQUE)
3. social_buttons (at TEXT NOT NULL, name TEXT NOT NULL, link TEXT PRIMARY KEY NOT NULL UNIQUE)
> If you need any help understanding what each column/row does check out the documentation [here](https://github.com/Andrea-moth/About-me/blob/main/src/lib.rs)

Then run `cargo build --release`, move ./target/release/server and database.db to the directory where you intend to keep your files and run `RUST_LOG=info ./server`

[^1]: I do not know why this is needed, only that it is. If you have any ideas please let me know.
