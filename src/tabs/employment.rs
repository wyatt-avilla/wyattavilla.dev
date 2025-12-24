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
pub fn EmploymentItem(employment_data: EmploymentData) -> impl IntoView {
    let styler_class = style! { "EmploymentItem",
        .employment-item {
            margin-bottom: 1.5rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid #787878;
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
        }

        .job-title {
            font-weight: 600;
            font-size: 1.4vmin;
        }

        .date-range {
            font-size: 1.1vmin;
        }

        .company-info {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: 0.5rem;
        }

        .company-name {
            font-weight: 500;
            font-size: 1.3vmin;
        }

        .location {
            font-size: 1.2vmin;
            color: #666;
        }

        .description-list {
            list-style-type: disc;
            margin-left: 1.5rem;
            margin-top: 0.5rem;
            font-size: 1.25vmin;
        }

        .description-list li {
            margin-bottom: 0.25rem;
        }

        .company-link {
            color: inherit;
            text-decoration: none;
        }

        .company-link:hover {
            text-decoration: underline;
        }
    };

    view! { class = styler_class,
        <div class="employment-item">
            <div class="employment-header">
                <div class="job-title">{employment_data.title}</div>
                <div class="date-range">{employment_data.start_date} " - " {employment_data.end_date}</div>
            </div>

            <div class="company-info">
                <a href={employment_data.link.clone()} class="company-link" target="_blank" rel="noopener noreferrer">
                    <span class="company-name">{employment_data.company}</span>
                </a>
                <span class="location">{employment_data.location}</span>
            </div>

            <ul class="description-list">
                {employment_data.description_bullets.into_iter().map(|bullet| {
                    view! {
                        <li>{bullet}</li>
                    }
                }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn Employment(employment_items: Vec<EmploymentData>) -> impl IntoView {
    let styler_class = style! { "Employment",
        .employment-section {
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }
    };

    view! { class = styler_class,
        <div class="employment-section">
            {employment_items.into_iter().map(|emp| {
                view! {
                    <EmploymentItem employment_data=emp.clone() />
                }
            }).collect_view()}
        </div>
    }
}
