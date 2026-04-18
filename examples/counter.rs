//! Example: Simple counter with accessible Checkbox and Button.
//!
//! Run with: cargo run --example counter

use gpui::*;
use gpui_rsx::rsx;
use stratum_gpui::{Button, Checkbox};

#[derive(Clone)]
struct AppModel {
    count: i32,
    enable_notifications: bool,
}

impl Render for AppModel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        rsx! {
            <div class="flex flex-col gap-4 p-4">
                <h1 class="text-2xl font-bold">{format!("Count: {}", self.count)}</h1>

                <div class="flex gap-2">
                    <button
                        class="px-4 py-2 bg-blue-500 text-white rounded-md"
                        on_click={cx.listener(|this, _evt, cx| {
                            this.count += 1;
                            cx.notify();
                        })}
                    >
                        {"Increment"}
                    </button>
                    <button
                        class="px-4 py-2 bg-red-500 text-white rounded-md"
                        on_click={cx.listener(|this, _evt, cx| {
                            this.count -= 1;
                            cx.notify();
                        })}
                    >
                        {"Decrement"}
                    </button>
                </div>

                <div class="flex items-center gap-2">
                    {Checkbox::bind(
                        |m: &AppModel| stratum_primitives::CheckboxState {
                            checked: stratum_core::TriState::from(m.enable_notifications),
                            id: "notify-checkbox".to_string(),
                        },
                        |m: &mut AppModel, state: stratum_primitives::CheckboxState| {
                            m.enable_notifications = state.checked.is_checked();
                        }
                    )}
                    <span>{"Enable notifications"}</span>
                </div>

                <div class="flex gap-2">
                    {Button::bind(
                        |m: &AppModel| stratum_primitives::PressableState {
                            pressed: false,
                            id: "action-btn".to_string(),
                        },
                        |m: &mut AppModel, _state: stratum_primitives::PressableState| {
                            println!("Button pressed! Count: {}", m.count);
                        }
                    )}
                    <span>{"Action Button (styled via GPUI)"}</span>
                </div>
            </div>
        }
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| AppModel {
                count: 0,
                enable_notifications: false,
            })
        });
    });
}
