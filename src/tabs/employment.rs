use leptos::prelude::*;
use stylers::style;

#[derive(Clone, Debug)]
pub struct EmploymentData {
    pub title: String,
    pub company: String,
    pub location: String,
    pub start_date: String,
    pub end_date: String,
    pub link: String,
    pub description_bullets: Vec<String>,
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn EmploymentItem(employment_data: EmploymentData) -> impl IntoView {
    let styler_class = style! { "EmploymentItem",
        .employment-item {
            margin-bottom: 1.5rem;
            padding-bottom: 1.5rem;
            border-bottom: 0.0625rem solid #787878;
        }

        .employment-item:last-child {
            border-bottom: none;
        }

        .employment-header {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: 0.5rem;
            flex-wrap: wrap;
            gap: 0.5rem;
        }

        .job-title {
            font-weight: 600;
            font-size: 1.125rem;
        }

        .date-range {
            font-size: 0.875rem;
            color: #555;
        }

        .company-info {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: 0.5rem;
            flex-wrap: wrap;
            gap: 0.5rem;
        }

        .company-name {
            font-weight: 500;
            font-size: 1rem;
        }

        .location {
            font-size: 0.875rem;
            color: #666;
        }

        .description-list {
            list-style-type: disc;
            margin-left: 1.5rem;
            margin-top: 0.5rem;
            font-size: 0.9rem;
            line-height: 1.5;
        }

        .description-list li {
            margin-bottom: 0.375rem;
        }

        .company-link {
            color: inherit;
            text-decoration: none;
        }

        .company-link:hover {
            text-decoration: underline;
        }

        @media (max-width: 768px) {
            .job-title {
                font-size: 1rem;
            }

            .date-range {
                font-size: 0.8rem;
            }

            .company-name {
                font-size: 0.9rem;
            }

            .location {
                font-size: 0.8rem;
            }

            .description-list {
                font-size: 0.85rem;
                margin-left: 1.25rem;
            }
        }

        @media (max-width: 480px) {
            .employment-item {
                margin-bottom: 1rem;
                padding-bottom: 1rem;
            }

            .employment-header {
                flex-direction: column;
                align-items: flex-start;
            }

            .company-info {
                flex-direction: column;
                align-items: flex-start;
            }

            .job-title {
                font-size: 0.95rem;
            }

            .date-range {
                font-size: 0.75rem;
            }

            .description-list {
                margin-left: 1rem;
                font-size: 0.8rem;
            }
        }
    };

    view! { class=styler_class,
        <div class="employment-item">
            <div class="employment-header">
                <div class="job-title">{employment_data.title}</div>
                <div class="date-range">
                    {employment_data.start_date} " - " {employment_data.end_date}
                </div>
            </div>

            <div class="company-info">
                <a
                    href=employment_data.link.clone()
                    class="company-link"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <span class="company-name">{employment_data.company}</span>
                </a>
                <span class="location">{employment_data.location}</span>
            </div>

            <ul class="description-list">
                {employment_data
                    .description_bullets
                    .into_iter()
                    .map(|bullet| {
                        view! { <li>{bullet}</li> }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn Employment(employment_items: Vec<EmploymentData>) -> impl IntoView {
    let styler_class = style! { "Employment",
        .employment-section {
            max-width: 50rem;
            margin: 0 auto;
            padding: 1.5rem;
        }

        @media (max-width: 768px) {
            .employment-section {
                padding: 1rem;
            }
        }

        @media (max-width: 480px) {
            .employment-section {
                padding: 0.75rem;
            }
        }
    };

    view! { class=styler_class,
        <div class="employment-section">
            {employment_items
                .into_iter()
                .map(|emp| {
                    view! { <EmploymentItem employment_data=emp.clone() /> }
                })
                .collect_view()}
        </div>
    }
}
