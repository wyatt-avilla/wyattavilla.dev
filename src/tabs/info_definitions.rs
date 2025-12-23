use super::education::EducationData;
use std::sync::LazyLock;

pub static EDUCATION_ITEMS: LazyLock<Vec<EducationData>> = LazyLock::new(|| {
    vec![
        EducationData {
            university: String::from("San Jose State University"),
            major: String::from("Software Engineering"),
            minor: None,
            specialization: String::from("Networking Software"),
            start_date: String::from("August 2025"),
            end_date: String::from("Present"),
            gpa: None,
            transcript_link: None,
        },
        EducationData {
            university: String::from("University of California, Santa Cruz"),
            major: String::from("Cognitive Science"),
            minor: Some(String::from("Computer Science")),
            specialization: String::from("AI and HCI"),
            start_date: String::from("September 2021"),
            end_date: String::from("June 2025"),
            gpa: Some(3.9),
            transcript_link: Some(String::from(
                "https://github.com/wyatt-avilla/resume/blob/main/assets/ucsc_official_transcript.pdf",
            )),
        },
    ]
});
