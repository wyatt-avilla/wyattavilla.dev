use crate::tabs::Tab;
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Tabs(active_tab: RwSignal<&'static Tab>) -> impl IntoView {
    let styler_class = style! { "Tabs",
        .tabs {
            display: flex;
            background: #0000AA;
            border-bottom: 2px solid #FFFFFF;
        }

        .tab {
            padding: 4px 16px;
            cursor: pointer;
            color: #FFFFFF;
            background: #0000AA;
            border-right: 2px solid #FFFFFF;
        }

        .active {
            background: #AAAAAA;
            color: #000000;
        }

        .tab:hover {
            background: #5555FF;
        }
    };

    view! { class = styler_class,
      <div class="tabs">
          <div
              class="tab"
              class:active=move || matches!(active_tab.get(), Tab::Main)
              on:click=move |_| active_tab.set(&Tab::Main)
          >
              {Tab::Main.to_string()}
          </div>
          <div
              class="tab"
              class:active=move || matches!(active_tab.get(), Tab::Advanced)
              on:click=move |_| active_tab.set(&Tab::Advanced)
          >
              {Tab::Advanced.to_string()}
          </div>
          <div
              class="tab"
              class:active=move || matches!(active_tab.get(), Tab::Security)
              on:click=move |_| active_tab.set(&Tab::Security)
          >
              {Tab::Security.to_string()}
          </div>
          <div
              class="tab"
              class:active=move || matches!(active_tab.get(), Tab::Boot)
              on:click=move |_| active_tab.set(&Tab::Boot)
          >
              {Tab::Boot.to_string()}
          </div>
          <div
              class="tab"
              class:active=move || matches!(active_tab.get(), Tab::Exit)
              on:click=move |_| active_tab.set(&Tab::Exit)
          >
              {Tab::Exit.to_string()}
          </div>
      </div>
    }
}
