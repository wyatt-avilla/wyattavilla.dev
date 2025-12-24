use super::info_definitions::{EDUCATION_ITEMS, EMPLOYMENT_ITEMS, PROJECT_ITEMS};
use crate::tabs::{About, Education, Employment, Projects};
use leptos::prelude::*;
use strum::Display;
use stylers::style;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Tab {
    About,
    Employment,
    Education,
    Projects,
}

impl Tab {
    pub fn all() -> &'static [Tab] {
        &[Tab::About, Tab::Employment, Tab::Education, Tab::Projects]
    }

    pub fn content(self) -> impl IntoView {
        view! {
            {move || match self {
                Tab::About => About().into_any(),
                Tab::Employment => {
                    view! { <Employment employment_items=EMPLOYMENT_ITEMS.clone() /> }.into_any()
                }
                Tab::Education => {
                    view! { <Education education_items=EDUCATION_ITEMS.clone() /> }.into_any()
                }
                Tab::Projects => {
                    view! { <Projects project_items=PROJECT_ITEMS.clone() /> }.into_any()
                }
            }}
        }
    }
}

#[component]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn TabComponent(tab: &'static Tab, active_tab: RwSignal<&'static Tab>) -> impl IntoView {
    let styler_class = style! { "TabComponent",
        .tab {
            padding: 0.5rem 1rem;
            cursor: pointer;
            color: #FFFFFF;
            background: #787878;
            border-right: 0.125rem solid #FFFFFF;
            font-size: 1.5rem;
            transition: background-color 0.2s;
            text-align: center;
        }

        .tab.active {
            background: #AAAAAA;
            color: #000000;
        }

        .tab:hover {
            background: #5555FF;
        }

        @media (max-width: 768px) {
            .tab {
                padding: 0.4rem 0.75rem;
                font-size: 0.9rem;
            }
        }

        @media (max-width: 480px) {
            .tab {
                padding: 0.35rem 0.5rem;
                font-size: 0.8rem;
            }
        }
    };

    view! {
        <div
            class=format!("tab {}", styler_class)
            class:active=move || active_tab.get() == tab
            on:click=move |_| active_tab.set(tab)
        >
            {tab.to_string()}
        </div>
    }
}
