use crate::footer::Footer;
use crate::header::Header;
use crate::tabs::{Tab, TabContent, Tabs};
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn App() -> impl IntoView {
    let active_tab = RwSignal::new(&Tab::About);

    let styler_class = style! { "App",
        @font-face {
            font-family: "Perfect DOS VGA";
            src: url("/assets/Perfect_DOS_VGA_437.ttf") format("truetype");
            font-weight: normal;
            font-style: normal;
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        :deep(html, body) {
            margin: 0;
            padding: 0;
            overflow: hidden;
            width: 100%;
            height: 100%;
            background: #000;
        }

        .bios-container {
            font-family: "Perfect DOS VGA", monospace;
            font-size: 2vmin;
            width: 100vw;
            height: 100vh;
            background: #0000AA;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }

        .content-box {
            width: 80vw;
            height: 80vw;
            max-height: 80vh;
            max-width: 80vh;
            margin: auto;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }

        @media (max-width: 768px) {
            .content-box {
                width: 90vw;
                height: 90vw;
                max-height: 90vh;
            }
        }
    };

    view! { class = styler_class,
        <div class="bios-container">
            <Header />

            <div class="content-box">
                <Tabs active_tab />
                <TabContent active_tab />
            </div>

            <Footer />
        </div>
    }
}
