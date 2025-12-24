use leptos::prelude::*;
use stylers::style;

#[derive(Clone, Debug)]
pub struct EducationData {
    pub university: String,
    pub major: String,
    pub minor: Option<String>,
    pub specialization: String,
    pub start_date: String,
    pub end_date: String,
    pub gpa: Option<f32>,
    pub transcript_link: Option<String>,
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn EducationItem(education_data: EducationData) -> impl IntoView {
    let styler_class = style! { "EducationItem",
        .education-item {
            margin-bottom: 1.5rem;
            padding-bottom: 1.5rem;
            border-bottom: 0.0625rem solid #787878;
        }

        .education-item:last-child {
            border-bottom: none;
        }

        .education-header {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: 0.5rem;
            flex-wrap: wrap;
            gap: 0.5rem;
        }

        .university-name {
            font-weight: 600;
            font-size: 1.125rem;
        }

        .date-range {
            font-size: 0.875rem;
            color: #555;
        }

        .degree-info {
            margin-bottom: 0.25rem;
        }

        .major {
            font-weight: 500;
            font-size: 1rem;
            margin-bottom: 0.25rem;
        }

        .minor, .specialization {
            font-size: 0.9rem;
            margin-bottom: 0.125rem;
        }

        .education-footer {
            display: flex;
            gap: 1rem;
            margin-top: 0.5rem;
            font-size: 0.875rem;
            flex-wrap: wrap;
        }

        @media (max-width: 768px) {
            .university-name {
                font-size: 1rem;
            }

            .date-range {
                font-size: 0.8rem;
            }

            .major {
                font-size: 0.9rem;
            }

            .minor, .specialization {
                font-size: 0.85rem;
            }

            .education-footer {
                font-size: 0.8rem;
            }
        }

        @media (max-width: 480px) {
            .education-item {
                margin-bottom: 1rem;
                padding-bottom: 1rem;
            }

            .education-header {
                flex-direction: column;
                align-items: flex-start;
            }

            .university-name {
                font-size: 0.95rem;
            }

            .date-range {
                font-size: 0.75rem;
            }
        }
    };

    view! { class=styler_class,
        <div class="education-item">
            <div class="education-header">
                <div class="university-name">{education_data.university}</div>
                <div class="date-range">
                    {education_data.start_date} " - " {education_data.end_date}
                </div>
            </div>

            <div class="degree-info">
                <div class="major">{education_data.major}</div>
                <div class="minor">
                    {education_data
                        .minor
                        .map(|m| {
                            view! {
                                "Minor: "
                                {m}
                            }
                        })}
                </div>
                <div class="specialization">"Specialization: " {education_data.specialization}</div>
            </div>

            <div class="education-footer">
                {education_data
                    .gpa
                    .map(|g| view! { <div class="gpa">"GPA: " {format!("{g:.2}")}</div> })}
                {education_data
                    .transcript_link
                    .map(|link| {
                        view! {
                            <a
                                href=link
                                class="transcript-link"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "View Transcript"
                            </a>
                        }
                    })}
            </div>
        </div>
    }
}

#[component]
pub fn Education(education_items: Vec<EducationData>) -> impl IntoView {
    let styler_class = style! { "Education",
        .education-section {
            max-width: 50rem;
            margin: 0 auto;
            padding: 1.5rem;
        }

        @media (max-width: 768px) {
            .education-section {
                padding: 1rem;
            }
        }

        @media (max-width: 480px) {
            .education-section {
                padding: 0.75rem;
            }
        }
    };

    view! { class=styler_class,
        <div class="education-section">
            {education_items
                .into_iter()
                .map(|edu| {
                    view! { <EducationItem education_data=edu.clone() /> }
                })
                .collect_view()}
        </div>
    }
}
