## Frontend 

This is where all the database stuff goes


## Setting up the database

You can setup the database by running these three commands in sqlite [^1]
```
CREATE TABLE charities(short TEXT PRIMARY KEY NOT NULL UNIQUE, flavor_text TEXT NOT NULL, link_text TEXT NOT NULL, link TEXT NOT NULL UNIQUE);
CREATE TABLE social_buttons(short TEXT PRIMARY KEY NOT NULL UNIQUE, at TEXT NOT NULL, name TEXT NOT NULL, link TEXT NOT NULL UNIQUE);
CREATE TABLE projects(short TEXT PRIMARY KEY NOT NULL UNIQUE, alt TEXT NOT NULL UNIQUE, name TEXT NOT NULL UNIQUE, image_path TEXT NOT NULL UNIQUE, description TEXT NOT NULL UNIQUE);
```

## How the database is used

* Projects

```html
<a class={ "project" } id={ short } href={ short }>
    <title class="name" >
        { name }
    </title>
    <img class= { "icon" } src={ format!("/assets/projects/{}", image_path ) } alt={ alt }/>
    <article class={ "description" } >
        { description }
    </article>
</a>
```

* charites 

```html
<h2 id={ "charity" }>
    { flavor_text }
    <a href={ link }>{ link_text }</a>
</h2>
```

* social_buttons

```html
<a id={ name } href={ link }>
    <img class= { "favicon" } src={ format!("https://icons.duckduckgo.com/ip3/{}.ico", name) }/>
    <span>{ at }</span>
</a>
```

## Building

Run `cargo build --release`, move ./target/release/server and database.db to the directory where you intend to keep your files and run `RUST_LOG=info ./server`

[^1]: Make sure the database is named projects.db, I will add configuration is future
