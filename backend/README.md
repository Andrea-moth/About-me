## Frontend 

This is where all the databases go

## Building

Make sure that there is a file named projects.db in both the root directory of the project and the backend folder[^1] 

You can setup the database by running these three commands in sqlite 
```
CREATE TABLE charities(short TEXT PRIMARY KEY NOT NULL UNIQUE, flavor_text TEXT NOT NULL, link_text TEXT NOT NULL, link TEXT NOT NULL UNIQUE);
CREATE TABLE social_buttons(short TEXT PRIMARY KEY NOT NULL UNIQUE, at TEXT NOT NULL, name TEXT NOT NULL, link TEXT NOT NULL UNIQUE);
CREATE TABLE projects(short TEXT PRIMARY KEY NOT NULL UNIQUE, alt TEXT NOT NULL UNIQUE, name TEXT NOT NULL UNIQUE, image_path TEXT NOT NULL UNIQUE, description TEXT NOT NULL UNIQUE);
```
> If you need any help understanding what each column/row does check out the documentation [here](https://github.com/Andrea-moth/About-me/blob/main/src/lib.rs)

Then run `cargo build --release`, move ./target/release/server and database.db to the directory where you intend to keep your files and run `RUST_LOG=info ./server`

[^1]: I do not know why this is needed, only that it is. If you have any ideas please let me know.
