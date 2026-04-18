use gpui::prelude::*;
use gpui::*;
use gpui_rsx::rsx;

pub struct Switch;

impl Switch {
    pub fn render<V: 'static>(
        checked: bool,
        on_change: impl Fn(bool) + 'static,
        cx: &mut Context<V>,
    ) -> impl IntoElement {
        let id = format!("switch-{}", uuid::Uuid::new_v4());
        rsx! {
            <div
                id={id}
                class="h-5 w-9 rounded-full p-0.5 cursor-pointer"
                when={(checked, |el: Stateful<Div>| el.bg(rgb(0x3b82f6)))}
                when={(!checked, |el: Stateful<Div>| el.bg(rgb(0xd1d5db)))}
                on_click={cx.listener(move |_, _: &ClickEvent, _window, _cx| {
                    on_change(!checked);
                })}
            >
                <div
                    class="h-4 w-4 rounded-full bg-white"
                    when={(checked, |el: Div| el.ml(px(16.0)))}
                />
            </div>
        }
    }
}
