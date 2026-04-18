use gpui::*;
use gpui_rsx::rsx;

pub struct Switch;

impl Switch {
    pub fn render(
        checked: bool,
        on_change: impl Fn(bool) + 'static,
        cx: &mut ViewContext<impl 'static>,
    ) -> Stateful<Div> {
        let id = format!("switch-{}", uuid::Uuid::new_v4());
        rsx! {
            <div
                id={id}
                role="switch"
                aria_checked={checked.to_string()}
                class={format!("h-5 w-9 rounded-full p-0.5 transition-colors {}", if checked { "bg-blue-500" } else { "bg-gray-300" })}
                on_click={cx.listener(move |_, _: &ClickEvent, _| {
                    on_change(!checked);
                })}
            >
                <div
                    class={format!("h-4 w-4 rounded-full bg-white shadow transform transition-transform {}", if checked { "translate-x-4" } else { "translate-x-0" })}
                />
            </div>
        }
    }
}
