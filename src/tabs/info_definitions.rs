use super::education::EducationData;
use super::employment::EmploymentData;
use super::projects::ProjectData;
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
            gpa: Some(4.0),
            transcript_link: Some(String::from(
                "https://github.com/wyatt-avilla/resume/blob/main/assets/sjsu_unofficial_transcript.pdf",
            )),
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

pub static PROJECT_ITEMS: LazyLock<Vec<ProjectData>> = LazyLock::new(|| {
    vec![
        ProjectData {
            title: String::from("Discord Bot with LLM Tool-Calling Integration"),
            link: String::from("https://github.com/wyatt-avilla/claude-discord-bot"),
            date: String::from("August 2025"),
            description: String::from(
                "Built a Discord bot in Rust enabling Anthropic’s Claude to take actions in Discord servers through tool calling, featuring image analysis, message reactions, and configurable interaction patterns with per-server configuration management and probabilistic autonomous responses. Exposed as a NixOS service and packaged with Nix flakes.",
            ),
        },
        ProjectData {
            title: String::from("Type-Safe REST API with ESP32 Client Integration"),
            link: String::from("https://github.com/wyatt-avilla/hypha"),
            date: String::from("June 2025"),
            description: String::from(
                "Built a production-ready REST API in Rust using Actix Web to monitor systemd service statuses, with shared type definitions ensuring compile-time safety between server and ESP32 client firmware. Implemented asynchronous message-passing architecture using Embassy for low-power consumption, packaged as a configurable NixOS service with CLI interface for real-time system monitoring.",
            ),
        },
    ]
});

pub static EMPLOYMENT_ITEMS: LazyLock<Vec<EmploymentData>> = LazyLock::new(|| {
    vec![
        EmploymentData {
            title: String::from("Software Engineer Intern"),
            company: String::from("Circuit Breaker Labs"),
            location: String::from("Washington, DC. Remote"),
            start_date: String::from("October 2025"),
            end_date: String::from("Present"),
            link: String::from("https://www.linkedin.com/company/circuit-breaker-labs-ai/"),
            description_bullets: vec![
                String::from("Architected and deployed a production FastAPI REST API with authentication for paid customers to execute LLM red-teaming tests, provisioned on NixOS with PostgreSQL backend"),
                String::from("Developed Python client library and GitHub Actions workflows to programmatically interface with the red-teaming API, enabling automated security testing in CI/CD pipelines"),
                String::from("Configured full-stack infrastructure deployment on VPS using NixOS declarative configuration, managing PostgreSQL database, API service, Nginx reverse proxy, and authentication layer"),
            ],
        },
        EmploymentData {
            title: String::from("Programming Course Developer & Tutor"),
            company: String::from("University of California, Santa Cruz"),
            location: String::from("Santa Cruz, California"),
            start_date: String::from("July 2024"),
            end_date: String::from("June 2025"),
            link: String::from("https://www.linkedin.com/school/ucsc/"),
            description_bullets: vec![
                String::from(
                    "Independently migrated 4 programming assignments and 12 assessments (∼2,000 lines) from C++ to Python, ensuring 100% test compatibility across 1,600+ test cases while implementing modern Python idioms including static typing, generics, comprehensions, and lazy evaluation",
                ),
                String::from(
                    "Led group tutoring sessions for data structures and algorithms, focusing on problem-solving strategies for technical interview preparation",
                ),
            ],
        },
        EmploymentData {
            title: String::from("Backend Developer Intern"),
            company: String::from("Lillup"),
            location: String::from("San Francisco, CA. Remote"),
            start_date: String::from("September 2024"),
            end_date: String::from("December 2024"),
            link: String::from("https://www.linkedin.com/company/lillup/"),
            description_bullets: vec![
                String::from(
                    "Built a FastAPI backend with LangChain integration for LLM tool calling, enabling the model to dynamically
query user data through a RESTful endpoint that processed multi-turn conversations with tool execution"
                ),
                String::from(
                    "Designed and implemented custom tool definitions with automated response parsing, handling JSON deserialization of LLM outputs into function arguments and returning structured responses",
                ),
                String::from(
                    "Extended Markdown syntax using markdown-it-py and regex parsing to support structured metadata (tags, progress indicators, due dates) for improved LLM context in time-sensitive applications",
                ),
                String::from(
                    "Established code quality standards with static typing (Mypy), Ruff for formatting/linting, and automated CI/CD pipeline through GitHub Actions",
                ),
            ],
        },
    ]
});
