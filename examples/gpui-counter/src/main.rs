use gpui::*;
use gpui_platform::application;
use gpui_rsx::rsx;
use stratum_gpui::Checkbox;

struct AppModel {
    count: i32,
    enable_notifications: bool,
}

impl Render for AppModel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.weak_entity();

        rsx! {
            <div class="flex flex-col gap-4 p-4 bg-white">
                <div class="text-2xl" font_weight={FontWeight::BOLD}>
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
                    {Checkbox::render(
                        self.enable_notifications,
                        {
                            let view = view.clone();
                            move |_window, cx| {
                                let view = view.clone();
                                // Defer the update to avoid re-entrant panic
                                cx.defer(move |cx| {
                                    if let Some(view) = view.upgrade() {
                                        view.update(cx, |model: &mut AppModel, cx| {
                                            model.enable_notifications = !model.enable_notifications;
                                            cx.notify();
                                            println!("Model updated: {}", model.enable_notifications);
                                        });
                                    }
                                });
                            }
                        },
                        cx,
                    )}
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
