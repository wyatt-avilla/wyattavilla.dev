use leptos::prelude::*;
use stylers::style;

#[derive(Clone, Debug)]
pub struct ProjectData {
    pub title: String,
    pub link: String,
    pub date: String,
    pub description: String,
}

#[component]
pub fn ProjectItem(project_data: ProjectData) -> impl IntoView {
    let styler_class = style! { "ProjectItem",
        .project-item {
            margin-bottom: 1.5rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid #787878;
        }

        .project-item:last-child {
            border-bottom: none;
        }

        .project-header {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: 0.5rem;
            flex-wrap: wrap;
        }

        .project-title {
            font-weight: 600;
            font-size: 1.4vmin;
        }

        .project-date {
            font-size: 1.1vmin;
        }

        .project-description {
            font-size: 1.2vmin;
            line-height: 1.5;
        }

        .project-link {
            color: inherit;
            text-decoration: none;
        }

        .project-link:hover {
            text-decoration: underline;
        }
    };

    view! { class = styler_class,
        <div class="project-item">
            <div class="project-header">
                <a href={project_data.link.clone()} class="project-link" target="_blank" rel="noopener noreferrer">
                    <div class="project-title">{project_data.title}</div>
                </a>
                <div class="project-date">{project_data.date}</div>
            </div>

            <div class="project-description">
                {project_data.description}
            </div>
        </div>
    }
}

#[component]
pub fn Projects(project_items: Vec<ProjectData>) -> impl IntoView {
    let styler_class = style! { "Projects",
        .projects-section {
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }
    };

    view! { class = styler_class,
        <div class="projects-section">
            {project_items.into_iter().map(|proj| {
                view! {
                    <ProjectItem project_data=proj.clone() />
                }
            }).collect_view()}
        </div>
    }
}
