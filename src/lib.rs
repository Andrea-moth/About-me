use serde::{Deserialize, Serialize};

/// Used as an intermediate for both the front and backend to parse and load projects
///
/// The name is used as the title of the project
/// The image_path corresponds to an image in the ../dist/projects directory
/// The description is used as the description of the project
///
/// In view it will look like this
///
/// ```html
/// <div class={ "project" } id={ project.name.clone() }>
///     <h2 class="name" id={ name }>
///         { name }
///     </h2>
///     <img class= { "icon" } src={ format!("./projects/{}", project.image_path ) }/>
///     <h3 class={ "description" } id={ description }>
///         { description }
///     </h3>
/// </div>
/// ```
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub image_path: String,
    pub description: String,
}

/// Used as an intermediate for both the front and backend to parse and load charities
///
/// The flavor_text is the text with no link attached
/// The link_text is the text with the link attached
/// The link is the link used
///
/// In view it will look like this
/// ```html
/// <h2>
///   { flavor_test }
///   <a href={ link }>{ link_text }</a>
/// </h2>
/// ```
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct Charity {
    pub flavor_text: String,
    pub link_text: String,
    pub link: String,
}
impl Charity {
    /// Used as a default for when the charity cannot be found in the database
    pub fn error() -> Self {
        Self {
            flavor_text: "Charity link not found".to_string(),
            link_text: "".to_string(),
            link: "".to_string(),
        }
    }
}

/// Used as an intermediate for both the front and backend to parse and load social links
///
/// The at is used for display
/// The name is used for the name of the site
/// The link is the link to the site
///
/// In view it will look like this
/// <a id={ name } href={ link }>
///     <img class="favicon" src={ format!("https://icons.duckduckgo.com/ip3/{}.ico", name ) }/>
///     <span>{ at }</span>
/// </a>
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct SocialLink {
    pub at: String,
    pub name: String,
    pub link: String,
}
