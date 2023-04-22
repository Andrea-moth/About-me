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

| short | alt | name | image_path | description |
| ----- | --- | ---- | ---------- | ----------- |
| Used as an indentifier, keep it to one word | The alt text for the image | The title of the project | The image used for the project, path starts at ./dist/assets/projects | The description of the path |

* charites 

| short | flavor_text | link_text | link | 
| ----- | ----------- | --------- | ---- |
| Used as an indentifier, keep it to one word | The text without the link | The text that makes up the link | The link to be used |

* social_buttons

| short | at | name | link | 
| ----- | -- | ---- | ---- |
| Used as an indentifier, keep it to one word | The name displayed | The name of the webiste being used [^1] | The link to be used |

[^1]: Make sure to include the domain. E.g, github.com instead of github
