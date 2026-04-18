use gpui::*;
use gpui_rsx::rsx;

pub struct Checkbox;

impl Checkbox {
    /// Render a checkbox with the given checked state and change callback.
    pub fn render(
        checked: bool,
        on_change: impl Fn(bool) + 'static,
        cx: &mut ViewContext<impl 'static>,
    ) -> Stateful<Div> {
        let id = format!("checkbox-{}", uuid::Uuid::new_v4());
        rsx! {
            <div
                id={id}
                role="checkbox"
                aria_checked={checked.to_string()}
                class="h-4 w-4 border rounded cursor-pointer"
                on_click={cx.listener(move |_, _: &ClickEvent, _| {
                    on_change(!checked);
                })}
            >
                {if checked { "✓" } else { "" }}
            </div>
        }
    }
}
