use gpui::prelude::*;
use gpui::*;
use gpui_rsx::rsx;

pub struct Checkbox;

impl Checkbox {
    pub fn render<V: 'static>(
        checked: bool,
        on_change: impl Fn(&mut Window, &mut App) + 'static,
        cx: &mut Context<V>,
    ) -> impl IntoElement {
        let id = format!("checkbox-{}", uuid::Uuid::new_v4());
        let focus_handle = cx.focus_handle();

        rsx! {
            <div
                class="p-1 cursor-pointer"
                on_click={cx.listener(move |_, _: &ClickEvent, window, cx| {
                    on_change(window, cx);
                })}
            >
                <div
                    id={id}
                    class="h-20 w-20 border-2 border-gray-500 rounded-md flex items-center justify-center text-xl"
                    bg={if checked { rgb(0x3b82f6) } else { rgb(0xffffff) }}
                    border_color={if checked { rgb(0x3b82f6) } else { rgb(0x6b7280) }}
                    track_focus={&focus_handle}
                >
                    {if checked { "✓" } else { "" }}
                </div>
            </div>
        }
    }
}
