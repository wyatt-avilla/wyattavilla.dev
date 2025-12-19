use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Footer() -> impl IntoView {
    let styler_class = style! { "Footer",
        .footer {
            background: #0000AA;
            color: #FFFFFF;
            padding: 8px 16px;
            border-top: 2px solid #FFFFFF;
            font-size: 12px;
            display: flex;
            justify-content: space-between;
        }
    };

    view! { class = styler_class,
      <div class="footer">
          <span>"v02.61 (C)Copyright 1985-2006, American Megatrends, Inc."</span>
          <span>"↑↓: Select Item | Enter: Go to Sub Screen | F10: Save and Exit | ESC: Exit"</span>
      </div>
    }
}
