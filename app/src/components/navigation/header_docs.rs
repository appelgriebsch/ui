use icons::Plus;
use leptos::prelude::*;
use registry::ui::button::{Button, ButtonSize};
use registry::ui::separator::{Separator, SeparatorOrientation};
use registry::ui::theme_toggle::ThemeToggle;

use crate::components::command_search_docs::CommandSearchDocs;
use crate::components::dioxus_link::DioxusLink;
use crate::components::github_stars::GithubStars;
use crate::components::navigation::nav_desktop::NavDesktop;
use crate::components::navigation::nav_mobile::NavMobile;

#[component]
pub fn HeaderDocs() -> impl IntoView {
    view! {
        // TODO: temporary, remove this banner once the Leptos vs Dioxus poll (#49) closes.
        <a
            href="https://github.com/rust-ui/ui/discussions/49"
            target="_blank"
            rel="noopener noreferrer"
            class="block sticky top-0 z-50 py-2 px-4 w-full text-sm text-center hover:underline bg-warning-light text-warning-dark underline-offset-2"
        >
            "🗳️ Leptos vs Dioxus: vote to decide the main framework (closes Aug 22) →"
        </a>

        <header class="sticky top-9 z-50 w-full border-b border-border/40 backdrop-blur supports-backdrop-filter:bg-background/60">
            <div class="container flex justify-between items-center px-6 h-14">
                <nav class="flex flex-1 justify-between items-center">
                    <div>
                        <NavMobile />
                        <NavDesktop />
                    </div>

                    <div class="flex gap-2 items-center min-w-0">
                        <CommandSearchDocs />
                        <Separator orientation=SeparatorOrientation::Vertical class="hidden ml-2 h-4 lg:block" />
                        <DioxusLink />
                        <GithubStars />
                        <Separator orientation=SeparatorOrientation::Vertical class="hidden h-4 lg:block" />
                        <ThemeToggle />
                        <Separator orientation=SeparatorOrientation::Vertical class="hidden h-4 lg:block" />
                        <Button href="/create" size=ButtonSize::Sm class="rounded-xl">
                            <Plus />
                            "New"
                        </Button>
                    </div>
                </nav>
            </div>
        </header>
    }
}
