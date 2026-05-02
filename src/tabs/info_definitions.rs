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
            title: String::from("Circuit Breaker Labs CLI"),
            link: String::from("https://github.com/circuitbreakerlabs/cli"),
            date: String::from("January 2026 - Present"),
            description: String::from(
                "Built and shipped cbl, the public Rust CLI for Circuit Breaker Labs' AI safety evaluation platform, connecting to a deployed FastAPI service over WebSockets with typed protocol envelopes, API-key authentication, and version negotiation. Drove evaluations through an async engine built on tokio::select! and JoinSet, with model providers abstracted behind a shared trait supporting OpenAI, Ollama, and Rhai-scripted custom integrations. Shipped Ratatui inline progress displays, 79 unit and integration tests, and cargo-dist releases targeting Apple Silicon, Intel macOS, Linux musl, and Windows MSVC.",
            ),
        },
        ProjectData {
            title: String::from("Discord Bot with LLM Tool-Calling Integration"),
            link: String::from("https://github.com/wyatt-avilla/claude-discord-bot"),
            date: String::from("August 2025"),
            description: String::from(
                "Built a Discord bot in Rust enabling Anthropic's Claude to take actions in Discord servers through tool calling, featuring image analysis, message reactions, and configurable interaction patterns with per-server configuration management and probabilistic autonomous responses. Exposed as a NixOS service and packaged with Nix flakes.",
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
        ProjectData {
            title: String::from("Open Source Contributor for Assembly Reverse Engineering"),
            link: String::from(
                "https://github.com/search?q=repo%3Adoldecomp%2Fmelee++author%3Awyatt-avilla&type=pullrequests&ref=advsearch",
            ),
            date: String::from("January 2024 - April 2024"),
            description: String::from(
                "Contributed 10 pull requests translating 2,800 lines of PowerPC assembly into 3,200 lines of C code for reverse-engineering Super Smash Bros. Melee. Ensured byte-perfect accuracy through GitHub Actions CI that validated the compiled binary against the original. Collaborated through code reviews with distributed team of developers.",
            ),
        },
    ]
});

pub static EMPLOYMENT_ITEMS: LazyLock<Vec<EmploymentData>> = LazyLock::new(|| {
    vec![
        EmploymentData {
            title: String::from("Member of the Technical Staff"),
            company: String::from("Circuit Breaker Labs"),
            location: String::from("Washington, DC. Remote"),
            start_date: String::from("October 2025"),
            end_date: String::from("Present"),
            link: String::from("https://www.linkedin.com/company/circuit-breaker-labs-ai/"),
            description_bullets: vec![
                String::from(
                    "Built Circuit Breaker Labs' FastAPI evaluation platform for AI safety red-teaming, owning typed REST and WebSocket endpoints for single-turn and multi-turn LLM evaluation workflows",
                ),
                String::from(
                    "Designed WebSocket evaluation flows with typed protocol envelopes, protocol-version validation, progress notifications, completion-request routing, and close-code error mapping, with database-backed API key authentication and monthly quota enforcement shared across REST and WebSocket handlers",
                ),
                String::from(
                    "Implemented model-provider call tracking across OpenAI, OpenRouter, and WebSocket client providers, capturing prompt contents, token counts, and errors to power precise API expenditure calculation and link generated tests and evaluated responses back to underlying model calls",
                ),
                String::from(
                    "Designed and migrated PostgreSQL schemas for users, API keys, test cases, generation records, test results, provider-call logs, and quotas using SQLAlchemy async and Alembic",
                ),
                String::from(
                    "Packaged and deployed the API with Nix flakes, uv2nix, a NixOS service module, systemd, PostgreSQL, agenix-managed secrets, nginx TLS/WebSocket proxying, and Prometheus/Alertmanager alerting, reducing build size from 17GB to 1GB and build time from 5+ hours to under a minute by removing CUDA dependencies and pinning Nix inputs",
                ),
                String::from(
                    "Refactored evaluation and provider functions to errors-as-values, enabling per-test-case error reporting, and debugged production-only request hanging caused by HTTP client timeout and resource-leak issues only surfacing after extended server uptime",
                ),
                String::from(
                    "Established CI/CD across Python and Nix codebases with GitHub Actions workflows for Ruff, strict Mypy, pytest coverage, Nix builds, statix, and nixfmt",
                ),
                String::from(
                    "Built the public Rust cbl CLI client for the evaluation platform, implementing async WebSocket orchestration, multi-provider abstraction, and cross-platform release automation (see Projects)",
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
                    "Built a FastAPI backend with LangChain integration for LLM tool calling, enabling the model to dynamically query user data through a RESTful endpoint that processed multi-turn conversations with tool execution",
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
    ]
});
