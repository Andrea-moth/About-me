use about_me::{Charity, Project, SocialLink};
use gloo_net::http::Request;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Properties, PartialEq)]
struct SocialButtonProp {
    button: SocialLink,
}

#[function_component(SocialButton)]
fn social_button(SocialButtonProp { button }: &SocialButtonProp) -> Html {
    html!(
        <>
            <a id={ button.name.clone() } href={ button.link.clone() }>
                <img class= { "favicon" } src={ format!("https://icons.duckduckgo.com/ip3/{}.ico", button.name.clone()) }/>
                <span>{ button.at.clone() }</span>
            </a>
        </>
    )
}

#[derive(Properties, PartialEq)]
struct SocialButtonsProp {
    buttons: Vec<SocialLink>,
}

#[function_component(SocialButtonList)]
fn social_button_list(SocialButtonsProp { buttons }: &SocialButtonsProp) -> Html {
    buttons
        .iter()
        .map(|button| {
            html!(
                <SocialButton button = { (*button).clone() }/>
            )
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct ProjectProp {
    project: Project,
}

#[function_component(ProjectBubble)]
fn project(ProjectProp { project }: &ProjectProp) -> Html {
    html!(
        <>
            <a class={ "project" } id={ project.short.clone() } href={ project.short.clone() }>
                <title class="name" >
                    { project.name.clone() }
                </title>
                <img class= { "icon" } src={ format!("/assets/projects/{}", project.image_path ) } alt={ project.alt.clone() }/>
                <article class={ "description" } >
                    { project.description.clone() }
                </article>
            </a>
        </>
    )
}

#[derive(Properties, PartialEq)]
struct ProjectsProp {
    projects: Vec<Project>,
}

#[function_component(ProjectList)]
fn project_list(ProjectsProp { projects }: &ProjectsProp) -> Html {
    projects
        .iter()
        .map(|project| {
            html!(
                <ProjectBubble project = { (*project).clone() }/>
            )
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct CharityProps {
    charity: Charity,
}

#[function_component(CharityLink)]
fn charity_link(CharityProps { charity }: &CharityProps) -> Html {
    html!(
        <>
            <h2 id={ "charity" }>
                { charity.flavor_text.clone() }
                <a href={ charity.link.clone() }>{ charity.link_text.clone() }</a>
            </h2>
        </>
    )
}

#[function_component(App)]
fn app() -> Html {
    let charity = use_state(|| Charity::default());
    let projects = use_state(|| vec![]);
    let social_buttons = use_state(|| vec![]);
    {
        let charity = charity.clone();
        let projects = projects.clone();
        let social_buttons = social_buttons.clone();
        use_effect_with_deps(
            move |_| {
                let charity = charity.clone();
                let projects = projects.clone();
                let social_buttons = social_buttons.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(response) = Request::get("/charities").send().await {
                        charity.set(response.json().await.unwrap_or(Charity::error()));
                    } else {
                        charity.set(Charity::error());
                    }

                    if let Ok(response) = Request::get("/projects").send().await {
                        projects.set(response.json().await.unwrap_or(vec![]));
                    } else {
                        projects.set(vec![]);
                    }

                    if let Ok(response) = Request::get("/social_buttons").send().await {
                        social_buttons.set(response.json().await.unwrap_or(vec![]));
                    } else {
                        social_buttons.set(vec![]);
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <div id="wrapper">
                <header>
                    <div id= { "name" }>
                        <h1>
                            { "Freyja-Moth" }
                        </h1>
                        <img class={ "pfp" } src={ "assets/pfp.png" } alt={ "A rounded icon of a picrew. She has brown eyes and hair, and is wearing a blue tank top with daisy earings and a daisy chain on her head. She is also holding a purple butterfly" }/>
                    </div>
                    <div id={ "site-row" }>
                        <SocialButtonList buttons={ (*social_buttons).clone() }/>
                    </div>
                </header>
                <div id={ "main-section" }>
                    <ProjectList projects={ (*projects).clone() }/>
                </div>
                <footer>
                    <CharityLink charity={ (*charity).clone() }/>
                </footer>
            </div>
        </>
    }
}
