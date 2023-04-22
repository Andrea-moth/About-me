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

## How the database is used

* Projects

```
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

```
<h2 id={ "charity" }>
    { flavor_text }
    <a href={ link }>{ link_text }</a>
</h2>
```

* social_buttons

```
<a id={ name } href={ link }>
    <img class= { "favicon" } src={ format!("https://icons.duckduckgo.com/ip3/{}.ico", name) }/>
    <span>{ at }</span>
</a>
```
