use crate::tabs::{Tab, TabComponent};
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Tabs(active_tab: RwSignal<&'static Tab>) -> impl IntoView {
    let styler_class = style! { "Tabs",
        .tabs {
            display: flex;
            background: #787878;
            border-bottom: 0.125rem solid #FFFFFF;
            overflow-x: auto;
        }

        .tabs::-webkit-scrollbar {
            display: none;
        }

        @media (max-width: 480px) {
            .tabs {
                justify-content: stretch;
            }

            .tabs > * {
                flex: 1;
            }
        }
    };

    view! { class=styler_class,
        <div class="tabs">
            {Tab::all()
                .iter()
                .map(|tab| {
                    view! { <TabComponent tab=tab active_tab=active_tab /> }
                })
                .collect_view()}
        </div>
    }
}
