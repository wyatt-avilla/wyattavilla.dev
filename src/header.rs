use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Header() -> impl IntoView {
    let styler_class = style! { "Header",
        .header {
            background: #0000AA;
            color: #FFFFFF;
            padding: 0.75rem 0;
            text-align: center;
            font-weight: bold;
            border-bottom: 0.125rem solid #FFFFFF;
            font-size: 1.125rem;
            letter-spacing: 0.05rem;
        }

        @media (max-width: 768px) {
            .header {
                padding: 0.6rem 0;
                font-size: 1.375rem;
                letter-spacing: 0.075rem;
            }
        }

        @media (max-width: 480px) {
            .header {
                padding: 0.75rem 0;
                font-size: 1.5rem;
                letter-spacing: 0.1rem;
            }
        }
    };

    view! { class=styler_class, <div class="header">"WYATT AVILLA"</div> }
}
