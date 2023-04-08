# About-me

The source code for my about me website, frontend stuff can be found [here](https://github.com/Andrea-moth/About-me/tree/main/frontend) and backend stuff [here](https://github.com/Andrea-moth/About-me/tree/main/backend)

After compiling and organizing it should look something like this

```
.
├── dist
│  └── ...
├── projects.db
└── server
```

## Database schema 

* Projects

| name | image_path | description |
| ---  | --- | --- | 
| TEXT PRIMARY KEY NOT NULL UNIQUE | TEXT NOT NULL UNIQUE | TEXT NOT NULL UNIQUE |

* charites 

| flavor_text | link_text | link | 
| --- | --- | --- |
| TEXT PRIMARY KEY NOT NULL | TEXT NOT NULL | TEXT NOT NULL UNIQUE |

* social_buttons

| at | link_text | link | 
| --- | --- | --- |
| TEXT NOT NULL | TEXT NOT NULL | TEXT PRIMARY KEY NOT NULL UNIQUE |
