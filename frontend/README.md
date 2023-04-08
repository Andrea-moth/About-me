## Frontend 

This is all the pretty stuff.

### Building

Before building make sure your directory looks like this

```
.
├── assets
│  ├── favicon.ico
│  ├── pfp.png
│  ├── projects
│  │  └── // Project previews
│  ├── style.css
│  └── vars.css
├── Cargo.toml
├── index.html
├── src
│  └── main.rs
└── Trunk.toml
```

Then run `trunk build` this will create a dist folder in the root of the project[^1]. Copy that into the folder where the server executable and the database can be found[^2] 


[^1]: The root of the about-me project, not the frontend. If you cannot find the dist folder try going back in your file explorer and looking there
[^2]: If you do not understand this have a look at the [backend readme](https://github.com/Andrea-moth/About-me/tree/main/backend/README.md)
