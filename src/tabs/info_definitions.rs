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
            title: String::from("Software Engineer Intern"),
            company: String::from("Principal Financial Group"),
            location: String::from("Des Moines, Iowa"),
            start_date: String::from("May 2026"),
            end_date: String::from("August 2026"),
            link: String::from("https://www.linkedin.com/company/principalfinancialgroup/"),
            description_bullets: vec![
                String::from(
                    "Extended the Digital Analytics team's TypeScript Helm chart-sync automation into a general transformation subsystem, parsing upstream charts, rendering templates, and patching every container definition to drop disallowed Linux kernel capabilities",
                ),
                String::from(
                    "Iteratively deployed transformed charts to the development environment and triaged live Kubernetes admission-controller policy reports, since policy evaluation required deployed resources rather than build-time chart checks, bringing all 51 development Kubernetes resources into compliance",
                ),
                String::from(
                    "Authored and reviewed a 6,000+-line downstream pull request demonstrating the maintenance cost of injecting the security patch, then wrote a technical recommendation for upstream chart maintainers to implement the fix in production",
                ),
                String::from(
                    "Shipped a further chart-sync extension that automatically created and cross-linked a ticket for every generated pull request, closing a manual tracking gap in the team's release workflow",
                ),
                String::from(
                    "Owned AWS infrastructure and CI quality gates for a five-person intern hackathon team that became a competition finalist, provisioning API Gateway, Lambda, Bedrock, and DynamoDB with AWS CDK behind a context-aware prompt-refinement VS Code extension",
                ),
                String::from(
                    "Built CI checks for testing, linting, formatting, and infrastructure validation for the completed extension, which was prepared for Visual Studio Marketplace publication",
                ),
                String::from(
                    "Deployed AWS CDK-managed SNS alerting for production CloudFront errors, triggering on three severity-threshold events within a 5-minute window, and migrated two CDK-backed applications to Principal's enterprise pipeline while replacing deprecated GitHub Actions steps",
                ),
                String::from(
                    "Diagnosed GitHub deployment failures to a repository merge-strategy misconfiguration, corrected it, and validated the fix across two core repositories",
                ),
                String::from(
                    "Integrated a session-analytics platform into a Next.js-based internal sandbox to validate customer-site tracking, and performed QA on a no-code A/B testing platform used by the team",
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
                    "Built a stateless FastAPI backend integrating LangChain with a self-hosted Llama model on AWS, enabling natural-language interaction with structured user profile data",
                ),
                String::from(
                    "Designed ~10 LangChain tool definitions for reading and updating profile fields, including typed JSON deserialization of tool-call outputs and structured response formatting",
                ),
                String::from(
                    "Implemented multi-turn conversation handling over a RESTful API, with clients retaining history and the backend executing LLM-selected tools per turn without server-side session state",
                ),
                String::from(
                    "Extended Markdown syntax via markdown-it-py to support structured progress and datetime metadata for LLM context injection, enabling human-writable task authoring",
                ),
                String::from(
                    "Established CI/CD from scratch with GitHub Actions, gating on pytest, Mypy, and Ruff; delivered the proof-of-concept to company leadership as sole backend engineer on a cross-platform team",
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
                    "Migrated 4 programming assignments and 12 assessments (~2,000 lines) from C++ to Python for a ~300-student data structures and algorithms course, preserving compatibility with 1,600+ existing test cases and adopting modern Python idioms throughout",
                ),
                String::from(
                    "Customized Codio development environments by extending the underlying Docker image with gcc, gdb, and valgrind, eliminating local setup friction and providing a consistent toolchain across all student machines",
                ),
                String::from(
                    "Supported automated grading workflows built around structured stdin/stdout execution, output diffing, and result reporting via POST requests to an internal university grading API",
                ),
                String::from(
                    "Led group tutoring sessions (~10 students each) covering linked lists, AVL trees, BFS, N-Queens, file I/O, Makefiles, and debugging, with an emphasis on conceptual understanding over direct solutions",
                ),
                String::from(
                    "Co-authored a public course wiki linked from Canvas for ~300 students, covering data structures, algorithms, and clean programming practices",
                ),
            ],
        },
    ]
});
