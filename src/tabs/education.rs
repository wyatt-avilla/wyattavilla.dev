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
pub fn EducationItem(education_data: EducationData) -> impl IntoView {
    let styler_class = style! { "EducationItem",
        .education-item {
            margin-bottom: 1.5rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid #787878;
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
        }

        .university-name {
            font-weight: 600;
            font-size: 1.4vmin;
        }

        .date-range {
            font-size: 1.1vmin;
        }

        .degree-info {
            margin-bottom: 0.25rem;
        }

        .major {
            font-weight: 500;
        }

        .minor, .specialization {
            font-size: 1.3vmin;
        }

        .education-footer {
            display: flex;
            gap: 1rem;
            margin-top: 0.5rem;
            font-size: 1.2vmin;
        }
    };

    view! { class = styler_class,
        <div class="education-item">
            <div class="education-header">
                <div class="university-name">{education_data.university}</div>
                <div class="date-range">{education_data.start_date} " - " {education_data.end_date}</div>
            </div>

            <div class="degree-info">
                <div class="major">{education_data.major}</div>
                <div class="minor">{education_data.minor.map(|m| view! { "Minor: " {m} })}</div>
                <div class="specialization">"Specialization: " {education_data.specialization}</div>
            </div>

            <div class="education-footer">
                {education_data.gpa.map(|g| view! {
                    <div class="gpa">"GPA: " {format!("{g:.2}")}</div>
                })}
                {education_data.transcript_link.map(|link| view! {
                    <a href={link} class="transcript-link" target="_blank" rel="noopener noreferrer">
                        "View Transcript"
                    </a>
                })}
            </div>
        </div>
    }
}

#[component]
pub fn Education(education_items: Vec<EducationData>) -> impl IntoView {
    let styler_class = style! { "Education",
        .education-section {
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }

        .section-title {
            font-size: 1.5vmin;
            font-weight: 700;
            color: #111827;
            margin-bottom: 1.5rem;
            padding-bottom: 0.5rem;
        }
    };

    view! { class = styler_class,
        <div class="education-section">
            {education_items.into_iter().map(|edu| {
                view! {
                    <EducationItem education_data=edu.clone() />
                }
            }).collect_view()}
        </div>
    }
}
