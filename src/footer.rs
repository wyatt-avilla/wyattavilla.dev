use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Footer() -> impl IntoView {
    let styler_class = style! { "Footer",
        .footer {
            background: #0000AA;
            color: #FFFFFF;
            padding: 0.5rem 1rem;
            border-top: 0.125rem solid #FFFFFF;
            display: flex;
            justify-content: space-between;
            flex-wrap: wrap;
            gap: 0.5rem;
            font-size: 0.875rem;
        }

        @media (max-width: 768px) {
            .footer {
                padding: 0.4rem 0.75rem;
                font-size: 0.75rem;
            }
        }

        @media (max-width: 480px) {
            .footer {
                flex-direction: column;
                text-align: center;
                padding: 0.35rem 0.5rem;
                font-size: 0.7rem;
            }
        }
    };

    view! { class=styler_class,
        <div class="footer">
            <span>"v02.67 (C)Copyright 1985-2006, American Megatrends, Inc."</span>
            <span>"Enter: Go to Sub Screen | F10: Save and Exit | ESC: Exit"</span>
        </div>
    }
}
