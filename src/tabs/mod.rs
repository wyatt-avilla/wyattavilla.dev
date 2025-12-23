mod about;
mod content;
mod education;
mod employment;
mod info_definitions;
mod projects;
mod tab;
#[allow(clippy::module_inception)]
mod tabs;

pub use about::About;
pub use content::TabContent;
pub use education::Education;
pub use employment::Employment;
pub use projects::Projects;
pub use tab::{Tab, TabComponent};
pub use tabs::Tabs;
