use gpui::{prelude::*, *};
use gpui_platform::application;
use gpui_rsx::rsx;

struct AppModel {
    count: i32,
    enable_notifications: bool,
}

impl Render for AppModel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <div class="flex flex-col gap-4 p-4">
                <div class="text-2xl text-white" font_weight={FontWeight::BOLD}>
                    {format!("Count: {}", self.count)}
                </div>

                <div class="flex gap-2">
                    <div
                        class="px-4 py-2 bg-blue-500 text-white rounded-md cursor-pointer"
                        on_click={cx.listener(|this: &mut AppModel, _event, _window, _cx| {
                            this.count += 1;
                        })}
                    >
                        {"Increment"}
                    </div>
                    <div
                        class="px-4 py-2 bg-red-500 text-white rounded-md cursor-pointer"
                        on_click={cx.listener(|this: &mut AppModel, _event, _window, _cx| {
                            this.count -= 1;
                        })}
                    >
                        {"Decrement"}
                    </div>
                </div>

                <div class="flex items-center gap-2">
                    <div
                        id="notify-checkbox"
                        class="h-20 w-20 border rounded-md cursor-pointer flex items-center justify-center"
                        when={(self.enable_notifications, |el| el.bg(rgb(0x3b82f6)).border_color(rgb(0x3b82f6)))}
                        when={(!self.enable_notifications, |el| el.border_color(rgb(0x9ca3af)))}
                        on_click={cx.listener(|this: &mut AppModel, _event, _window, _cx| {
                            this.enable_notifications = !this.enable_notifications;
                        })}
                    >
                        {if self.enable_notifications { "✓" } else { "" }}
                    </div>
                    <span>{"Enable notifications"}</span>
                </div>
            </div>
        }
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| AppModel {
                count: 0,
                enable_notifications: false,
            })
        })
        .unwrap();
        cx.activate(true);
    });
}
