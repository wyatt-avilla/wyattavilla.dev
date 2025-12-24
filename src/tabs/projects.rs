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
            border-bottom: 0.0625rem solid #787878;
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
            gap: 0.5rem;
        }

        .project-title {
            font-weight: 600;
            font-size: 1.125rem;
        }

        .project-date {
            font-size: 0.875rem;
            color: #555;
        }

        .project-description {
            font-size: 0.9rem;
            line-height: 1.5;
        }

        .project-link {
            color: inherit;
            text-decoration: none;
        }

        .project-link:hover {
            text-decoration: underline;
        }

        @media (max-width: 768px) {
            .project-title {
                font-size: 1rem;
            }

            .project-date {
                font-size: 0.8rem;
            }

            .project-description {
                font-size: 0.85rem;
            }
        }

        @media (max-width: 480px) {
            .project-item {
                margin-bottom: 1rem;
                padding-bottom: 1rem;
            }

            .project-header {
                flex-direction: column;
                align-items: flex-start;
            }

            .project-title {
                font-size: 0.95rem;
            }

            .project-date {
                font-size: 0.75rem;
            }

            .project-description {
                font-size: 0.8rem;
                line-height: 1.4;
            }
        }
    };

    view! { class=styler_class,
        <div class="project-item">
            <div class="project-header">
                <a
                    href=project_data.link.clone()
                    class="project-link"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <div class="project-title">{project_data.title}</div>
                </a>
                <div class="project-date">{project_data.date}</div>
            </div>

            <div class="project-description">{project_data.description}</div>
        </div>
    }
}

#[component]
pub fn Projects(project_items: Vec<ProjectData>) -> impl IntoView {
    let styler_class = style! { "Projects",
        .projects-section {
            max-width: 50rem;
            margin: 0 auto;
            padding: 1.5rem;
        }

        @media (max-width: 768px) {
            .projects-section {
                padding: 1rem;
            }
        }

        @media (max-width: 480px) {
            .projects-section {
                padding: 0.75rem;
            }
        }
    };

    view! { class=styler_class,
        <div class="projects-section">
            {project_items
                .into_iter()
                .map(|proj| {
                    view! { <ProjectItem project_data=proj.clone() /> }
                })
                .collect_view()}
        </div>
    }
}
