# Stratum-GPUI: A Comprehensive Implementation Plan (Revised)

## Executive Summary

This document presents a complete, production‑ready plan for building `stratum-gpui`, a native desktop adapter that renders `stratum-primitives` components using the GPUI framework. The adapter interprets `RenderOutput` into a live GPUI element tree, reusing the exact same primitive logic as the Leptos and Dioxus adapters. The result is a single, cross‑framework component system that delivers true native performance on macOS, Windows, and Linux.

This revised plan incorporates:
- Detailed corrections from a thorough review of the GPUI source code.
- Integration of **`gpui-rsx`**, a mature JSX‑like macro, for internal UI components, test fixtures, and potential reuse of static lookup tables.
- A refined class‑mapping strategy that respects GPUI's actual API surface (no generic `.attr()`, proper `FocusHandle` usage, etc.).

---

## 1. Revised Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      Application Code                           │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     stratum-gpui (Adapter)                      │
│  ┌───────────────────┐  ┌───────────────────┐  ┌─────────────┐ │
│  │ RenderOutput →    │  │ ComponentEvent    │  │ThemeContext │ │
│  │ GPUI Element Tree │  │ ↔ GPUI Events     │  │ (Global)    │ │
│  └───────────────────┘  └───────────────────┘  └─────────────┘ │
│                                                                 │
│  Internal UI (prompts, overlays) built with gpui-rsx            │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    stratum-primitives                           │
│   (Pressable, Checkbox, Dialog, Tabs, Table, …)                │
└─────────────────────────────────────────────────────────────────┘
```

**Key Principle:** We do **not** render to a string. We interpret `RenderOutput` to construct a **live GPUI element tree** that behaves identically to its web counterpart.

**New Integration:** `gpui-rsx` is used for:
- Writing internal UI components (e.g., fallback prompt renderer) with minimal boilerplate.
- (Optional) Reusing its static lookup tables for Tailwind colors and attributes when resolving dynamic classes.
- Declarative test fixtures for visual regression and interaction tests.

---

## 2. Core Implementation Details (GPUI Corrected)

### 2.1 Converting `RenderOutput` to GPUI Elements

We recursively traverse `RenderOutput` and build a GPUI `AnyElement`. Each primitive becomes a `div()` with appropriate styling and behaviour, **corrected for GPUI's actual API**.

```rust
pub fn render_output_to_element(
    output: &RenderOutput,
    cx: &mut App,
) -> AnyElement {
    let mut element = div();

    // 1. Apply semantic identity (for debugging and potential web target)
    match output.effective_tag() {
        "button" => {
            element = element
                .id(ElementId::Name("button".into()))
                .cursor(CursorStyle::PointingHand);
        }
        "input" | "textarea" => {
            return build_text_input_element(output, cx);
        }
        "dialog" => {
            element = element.id(ElementId::Name("dialog".into()));
        }
        _ => {}
    }

    // 2. Map Tailwind classes to GPUI fluent methods (using theme‑aware resolver)
    for class in &output.classes {
        element = apply_class(element, class, cx);
    }

    // 3. Data attributes are stored in component state; no .attr() method exists
    //    (We can store them in the component's Entity for querying later if needed)

    // 4. Inline styles
    for (prop, val) in &output.styles {
        element = apply_style(element, prop, val);
    }

    // 5. Children
    match &output.children {
        ChildrenSpec::Text(text) => {
            element = element.child(text.clone());
        }
        ChildrenSpec::Elements(children) => {
            for child in children {
                element = element.child(render_output_to_element(child, cx));
            }
        }
        _ => {}
    }

    element.into_any_element()
}
```

#### Class Mapping (Partial List, Theme‑Aware)

```rust
fn apply_class(div: Div, class: &str, cx: &App) -> Div {
    // Use the same tables as gpui-rsx if exposed, or a vendored subset.
    match class {
        // Layout
        "flex" => div.flex(),
        "inline-flex" => div.inline_flex(),
        "flex-col" => div.flex_col(),
        "items-center" => div.items_center(),
        "justify-center" => div.justify_center(),
        "gap-1" => div.gap_1(),
        "gap-2" => div.gap_2(),

        // Spacing (numeric values)
        s if let Some(rest) = s.strip_prefix("gap-") => {
            if let Ok(n) = rest.parse::<f32>() {
                div.gap(px(n))
            } else {
                div
            }
        }
        s if let Some(rest) = s.strip_prefix("p-") => {
            if let Ok(n) = rest.parse::<f32>() {
                div.p(px(n))
            } else {
                div
            }
        }
        // ... other numeric prefixes (px-, py-, m-, w-, h-, etc.)

        // Colors (theme‑aware)
        "bg-primary" => div.bg(resolve_color("primary", cx)),
        "text-primary-foreground" => div.text_color(resolve_color("primary-foreground", cx)),
        "bg-background" => div.bg(resolve_color("background", cx)),
        "border-border" => div.border_color(resolve_color("border", cx)),

        // Tailwind color classes (if not using gpui-rsx tables, we can implement a subset)
        "bg-blue-500" => div.bg(rgb(0x3b82f6)),
        "text-red-600" => div.text_color(rgb(0xdc2626)),

        // Borders & Effects
        "rounded-md" => div.rounded_md(),
        "shadow-sm" => div.shadow_sm(),

        // Interactive
        "cursor-pointer" => div.cursor(CursorStyle::PointingHand),

        _ => div,
    }
}
```

**Key Corrections:**
- **No `.attr()`** – GPUI does not have a generic attribute system. Semantic roles are conveyed via `ElementId` or stored in component state.
- **No `.tab_index()` directly on `Div`** – Focus and tab order are managed via `FocusHandle` (obtained with `cx.focus_handle().tab_index(n).tab_stop(true)`). The element must use `.track_focus(&focus_handle)`.
- **Color resolution** is done via a global `ThemeContext` (see §2.6).

### 2.2 Event Translation

GPUI event listeners convert native events to `ComponentEvent` and invoke the primitive’s `on_event` method.

| GPUI Event              | `ComponentEvent`                        |
|-------------------------|-----------------------------------------|
| `ClickEvent`            | `ComponentEvent::Click`                 |
| `KeyDownEvent`          | `ComponentEvent::KeyDown` (mapped keys) |
| `FocusIn` / `FocusOut`  | `ComponentEvent::Focus` / `Blur`        |
| `MouseDown` / `MouseUp` | Used internally for press states        |

**Example: `Button` Component**

```rust
pub struct Button {
    props: ButtonProps,
    state: PressableState,
    focus_handle: FocusHandle,
}

impl Render for Button {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let output = Pressable::render(&self.props, &self.state);
        let element = render_output_to_element(&output, cx);

        element
            .track_focus(&self.focus_handle)
            .on_click(cx.listener(|this, _event, _window, cx| {
                let result = Pressable::on_event(
                    &this.props,
                    &mut this.state,
                    ComponentEvent::Click,
                );
                if result.state_changed {
                    cx.notify();
                }
            }))
            .on_key_down(cx.listener(|this, event, _window, cx| {
                if let Some(key) = map_gpui_key(&event.keystroke) {
                    let result = Pressable::on_event(
                        &this.props,
                        &mut this.state,
                        ComponentEvent::KeyDown {
                            key,
                            modifiers: map_modifiers(&event.modifiers),
                        },
                    );
                    if result.state_changed {
                        cx.notify();
                    }
                    if result.prevent_default {
                        cx.stop_propagation();
                    }
                }
            }))
    }
}

impl Focusable for Button {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
```

### 2.3 State Management with `Entity`

Every interactive primitive is wrapped in a GPUI `Entity` that implements `Render`. The entity owns:
- The primitive’s props.
- The primitive’s state (e.g., `PressableState`, `CheckboxState`).
- A `FocusHandle` (obtained via `cx.focus_handle()` and configured with appropriate `tab_index`).

When an event modifies the state, we call `cx.notify()` to trigger a re‑render.

### 2.4 Text Input (`TextInput` / `TextArea`)

This is the most complex primitive. We implement a custom GPUI `Element` (not a `div`) that:
- Uses `EntityInputHandler` for IME support.
- Renders the text, selection, and cursor.
- Syncs the primitive’s `TextInputState` (value, selection range, validation) with the GPUI input handler.
- Dispatches `InputEvent` to the primitive on every change.

The implementation is directly inspired by `gpui_component::input::Input`, adapted to use `stratum_primitives::TextInputState`.

### 2.5 Overlays (`Dialog`, `Popover`, `Tooltip`)

- **Popover / Tooltip:** Use GPUI’s `deferred` + `anchored` elements. The primitive’s open state controls whether the anchored element is rendered.
- **Dialog:** Render a backdrop (`div().bg(black().opacity(0.5))`) and an anchored, focus‑trapped container. Focus trap is achieved by storing the previously focused handle and restoring it on close.

### 2.6 Theme Integration

We provide a global `ThemeContext` that holds the active `stratum_theme::Theme`. A helper function resolves semantic color tokens to `Hsla`.

```rust
struct ThemeContext {
    theme: Arc<stratum_theme::Theme>,
    mode: ThemeMode,
}

impl Global for ThemeContext {}

fn resolve_color(token: &str, cx: &App) -> Hsla {
    let ctx = cx.global::<ThemeContext>();
    let palette = match ctx.mode {
        ThemeMode::Light => &ctx.theme.colors.light,
        ThemeMode::Dark => &ctx.theme.colors.dark,
    };
    match token {
        "primary" => palette.primary.into(),
        "primary-foreground" => palette.primary_foreground.into(),
        "background" => palette.background.into(),
        "border" => palette.border.into(),
        // … all tokens used by stratum-components
        _ => gpui::black(),
    }
}
```

---

## 3. Leveraging `gpui-rsx`

`gpui-rsx` is a mature, compile‑time JSX‑like macro for GPUI. It cannot replace the runtime conversion of `RenderOutput`, but it provides substantial value in three areas:

### 3.1 Internal UI Components (Prompts, Dialogs, Overlays)

The adapter must provide a fallback prompt renderer and may implement custom overlays. Writing these with `gpui-rsx` drastically reduces boilerplate.

```rust
use gpui_rsx::rsx;

fn render_prompt(message: &str, buttons: &[PromptButton], cx: &mut App) -> impl IntoElement {
    rsx! {
        <div class="fixed inset-0 flex items-center justify-center">
            <div class="absolute inset-0 bg-black opacity-50" />
            <div class="relative bg-white rounded-lg p-6 max-w-md">
                <p class="text-lg mb-4">{message}</p>
                <div class="flex justify-end gap-2">
                    {for button in buttons {
                        <button
                            key={button.label()}
                            class="px-4 py-2 rounded-md bg-blue-500 text-white"
                            onClick={cx.listener(move |_, _, cx| { /* handle click */ })}
                        >
                            {button.label()}
                        </button>
                    }}
                </div>
            </div>
        </div>
    }
}
```

### 3.2 Static Lookup Tables for Dynamic Classes

`gpui-rsx` internally maintains exhaustive Tailwind color palettes and attribute mappings. If these tables are exposed in a separate crate (e.g., `gpui-rsx-tables`), we can reuse them for dynamic class resolution, ensuring **100% consistency** between compile‑time and runtime styles.

**Ideal Integration:**
```rust
use gpui_rsx_tables::{lookup_color, lookup_spacing_method};

fn resolve_tailwind_class(class: &str) -> Option<Box<dyn Fn(Div) -> Div>> {
    if let Some(rest) = class.strip_prefix("text-") {
        if let Some(hex) = lookup_color(&rest.replace('-', "_")) {
            return Some(Box::new(move |d| d.text_color(rgb(hex))));
        }
    }
    // ... spacing, layout, etc.
    None
}
```

**Fallback:** If upstream separation is not yet available, we vendor a minimal subset of the tables (colors, spacing prefixes) into `stratum-gpui`. This is a pragmatic short‑term solution.

### 3.3 Declarative Test Fixtures

All visual regression and interaction tests for `stratum-gpui` will use `gpui-rsx` to render complex component trees, improving readability and maintainability.

```rust
#[gpui::test]
fn test_button_press(cx: &mut TestAppContext) {
    let view = cx.new(|_| MyButtonView::new());
    let window = cx.open_window(|_, cx| view.clone());
    
    cx.update(|cx| {
        rsx! {
            <div class="flex p-4">
                <MyButton id="btn" onClick={cx.listener(|_, _, _| {})} />
            </div>
        }
    });
    
    // Simulate click, assert state changes...
}
```

---

## 4. Component Implementation Roadmap

| Phase | Components                                                                 | Key Deliverables |
|-------|-----------------------------------------------------------------------------|------------------|
| **1** | `Button`, `Link`, `Divider`, `Spinner`, `VisuallyHidden`                    | Core `RenderOutput` → GPUI converter; class mapping foundation; theme integration. |
| **2** | `Checkbox`, `Radio`, `Switch`, `Slider`, `Rating`                           | Primitive state integration; custom styling for non‑text inputs. |
| **3** | `Input`, `Textarea`                                                          | Custom GPUI `Element` with IME support; full `TextInput` primitive integration. |
| **4** | `Dialog`, `AlertDialog`, `Popover`, `Tooltip`                               | `deferred` + `anchored` usage; focus trap implementation; internal UI with `gpui-rsx`. |
| **5** | `Tabs`, `Accordion`, `Menu`, `Select`, `Breadcrumb`, `Pagination`           | Composite component patterns; keyboard navigation. |
| **6** | `Table`, `List`, `Tree`, `VirtualList`                                      | Integration with GPUI’s `list` and `uniform_list` for virtual scrolling. |

**Phase 1** validates the entire adapter stack and produces a working button and link that look and behave correctly.

**Phase 3** is the most technically challenging; we leverage the proven `gpui_component::input` implementation as a template.

---

## 5. Challenges and Mitigations

| Challenge | Mitigation |
|-----------|------------|
| **GPUI lacks generic attributes** | ARIA/role are unnecessary for desktop; semantic identity is maintained via `ElementId`. |
| **Tab index management** | Use `FocusHandle` with `.tab_index()` and `.tab_stop(true)`, attached via `.track_focus()`. |
| **IME text input complexity** | Reuse the `EntityInputHandler` pattern from `gpui_component::input`. |
| **Dynamic class resolution** | Prefer static string literals. For dynamic expressions, reuse `gpui-rsx` tables (if exposed) or vendor a subset. |
| **Consistency between static and dynamic styles** | Use `gpui-rsx` for internal UI and (optionally) its tables for runtime class resolution. |
| **Focus trap for dialogs** | Manual focus management: store previous focus, move focus to dialog content, restore on close. |

---

## 6. Conclusion

This revised plan delivers a complete, production‑ready path to `stratum-gpui`. By interpreting `RenderOutput` into GPUI’s native element system, reusing the exact same `stratum-primitives` logic, and incorporating `gpui-rsx` for internal UI and table reuse, we achieve:

- **Full feature parity** with Leptos and Dioxus adapters.
- **Native look‑and‑feel** on macOS, Windows, and Linux.
- **Maintainable, shared business logic** across all platforms and frameworks.
- **Reduced boilerplate and improved developer experience** through `gpui-rsx`.

The implementation is phased to deliver value early and manage complexity. With the corrections from the GPUI codebase review and the new `gpui-rsx` integration, this plan is ready for execution.
