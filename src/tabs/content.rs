use crate::tabs::Tab;
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn TabContent(active_tab: RwSignal<&'static Tab>) -> impl IntoView {
    let styler_class = style! { "TabContent",
        .content {
            flex: 1;
            padding: 1rem;
            overflow-y: auto;
            background: #AAAAAA;
            color: #000000;
        }

        .content::-webkit-scrollbar {
            display: none;
        }

        @media (max-width: 768px) {
            .content {
                padding: 0.75rem;
            }
        }

        @media (max-width: 480px) {
            .content {
                padding: 0.5rem;
            }
        }
    };

    view! { class=styler_class, <div class="content">{move || active_tab.get().content()}</div> }
}
