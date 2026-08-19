use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    hooks::use_params_map,
    path,
};

use crate::canvas::{load_canvas, render_node};

#[derive(Clone)]
pub(crate) struct HoveredNodeContext {
    pub file: RwSignal<Option<String>>,
    pub position: RwSignal<Option<(isize, isize)>>,
    pub size: RwSignal<Option<(isize, isize)>>,
    pub fullscreen: RwSignal<bool>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let hover = HoveredNodeContext {
        file: RwSignal::new(None),
        position: RwSignal::new(None),
        size: RwSignal::new(None),
        fullscreen: RwSignal::new(false),
    };
    provide_context(hover.clone());

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio_website.css" />
        <Title text="Audrick Yeu | Portfolio" />

        <Router>
            <main>
                <SideBar />

                <Routes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=path!("/")
                        view=|| view! { <Redirect path="/canvas/Movie" /> }
                    />

                    <Route
                        path=path!("/canvas/:name")
                        view=ObsidianCanvas
                    />
                </Routes>

                <FootBar />
            </main>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link
                    rel="icon"
                    href="/favicon_light.svg"
                    type="image/svg+xml"
                    media="(prefers-color-scheme: light)"
                />
                <link
                    rel="icon"
                    href="/favicon_dark.svg"
                    type="image/svg+xml"
                    media="(prefers-color-scheme: dark)"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

#[component]
fn ObsidianCanvas() -> impl IntoView {
    let params = use_params_map();

    let canvas = Resource::new(
        move || params.read().get("name").unwrap_or_default(),
        |name| async move { load_canvas(name).await.unwrap() },
    );

    view! {
        <Suspense fallback=|| view! {
            <div class="loading">"Loading canvas..."</div>
        }>
            {move || {
                canvas.get().map(|canvas_file| {
                    view! {
                        <MouseTracker>
                            <FocusBox />
                            <div class="obsidian_canvas">
                                {canvas_file.nodes.into_iter().map(render_node).collect_view()}
                            </div>
                        </MouseTracker>
                    }
                })
            }}
        </Suspense>
    }
}

#[server]
pub async fn list_canvases() -> Result<Vec<String>, ServerFnError> {
    Ok(std::fs::read_dir("canvas")?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("canvas"))
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
        .collect())
}

#[component]
fn FootBar() -> impl IntoView {
    let hover = expect_context::<HoveredNodeContext>();
    view! {<footer>{move || {
        hover.file.get()
            .unwrap_or_else(|| "(੭｡╹▿╹｡)੭".into())
    }}</footer>}
}

#[component]
fn FocusBox() -> impl IntoView {
    let hover = expect_context::<HoveredNodeContext>();

    view! {
        <div
            class="focus-box"
            class:expand=move || hover.fullscreen.get()
            style=move || {
                let position = hover.position.get().unwrap_or_default();
                let size = hover.size.get().unwrap_or_default();

                format!(
                    "position:absolute;\
                     left:{}px;\
                     top:{}px;\
                     width:{}px;\
                     height:{}px;",
                    position.0,
                    position.1,
                    size.0,
                    size.1
                )
            }
        ></div>
    }
}

#[component]
fn SideBar() -> impl IntoView {
    use leptos_router::components::A;

    let canvases = Resource::new(
        || (),
        |_| async { list_canvases().await.unwrap_or_default() },
    );

    let (collapsed, set_collapsed) = signal(false);

    let toggle_collapse = move |_| {
        set_collapsed.update(|c| *c = !*c);
    };

    #[cfg(feature = "hydrate")]
    {
        use leptos_use::use_css_var;

        let (_, set_template) = use_css_var("--columns-template");

        Effect::new(move |_| {
            set_template.set(if collapsed.get() {
                "0fr 1fr".to_string()
            } else {
                "1fr 6fr".to_string()
            });
        });
    }

    let collapse_button = move || {
        view! {
            <button on:click=toggle_collapse>
                {move || {
                    if collapsed.get() {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <rect width="18" height="18" x="3" y="3" rx="2"/>
                                <path d="M9 3v18"/>
                                <path d="m14 9 3 3-3 3"/>
                            </svg>
                        }
                    } else {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <rect width="18" height="18" x="3" y="3" rx="2"/>
                                <path d="M9 3v18"/>
                                <path d="m16 15-3-3 3-3"/>
                            </svg>
                        }
                    }
                }}
            </button>
        }
    };

    let canvas_list = move || {
        canvases.get().map(|list| {
            view! {
                <ul>
                    <For
                        each=move || list.clone()
                        key=|name| name.clone()
                        children=move |name| {
                            view! {
                                <li>
                                    <A href=format!("/canvas/{name}")>
                                        {name}
                                    </A>
                                </li>
                            }
                        }
                    />
                </ul>
            }
        })
    };

    view! {
        <nav class:collapsed=move || collapsed.get()>
        {collapse_button}
            <Suspense fallback=|| view! {
                <ul>
                    <li>"Loading…"</li>
                </ul>
            }>
                {canvas_list}
            </Suspense>
        </nav>
    }
}

#[component]
fn MouseTracker(children: Children) -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        use leptos::{
            ev::{PointerEvent, WheelEvent},
            *,
        };
        use leptos_use::{
            use_css_var, use_mouse, use_window_size, UseMouseReturn, UseWindowSizeReturn,
        };
        use wasm_bindgen::JsCast;

        let panning = RwSignal::new(false);

        let offset = RwSignal::new(Point::default());
        let world_origin = RwSignal::new(Point::default());
        let pan_origin = RwSignal::new(Point::default());

        let pointer = RwSignal::new(Point::default());

        let scale = RwSignal::new(1.0_f64);

        let (css_var_scale, set_css_var_scale) = use_css_var("--scale");

        Effect::new(move |_| {
            set_css_var_scale.set(scale.get().to_string());
        });

        let UseMouseReturn {
            x: mouse_x,
            y: mouse_y,
            ..
        } = use_mouse();

        let UseWindowSizeReturn { width, height } = use_window_size();

        let on_wheel = move |ev: WheelEvent| {
            ev.prevent_default();

            let old_scale = scale.get();

            let zoom_factor = if ev.delta_y() < 0.0 { 1.1_f64 } else { 0.9_f64 };

            let new_scale = (old_scale * zoom_factor).clamp(0.01, 10.0);

            let old_offset = offset.get();

            let world_x = (mouse_x.get() - old_offset.x) / old_scale;
            let world_y = (mouse_y.get() - old_offset.y) / old_scale;

            offset.set(Point {
                x: mouse_x.get() - world_x * new_scale,
                y: mouse_y.get() - world_y * new_scale,
            });

            scale.set(new_scale);
        };

        let on_pointer_down = move |ev: PointerEvent| {
            ev.prevent_default();

            // Middle mouse button
            if ev.button() == 1 {
                if let Some(target) = ev.current_target() {
                    let element: web_sys::Element = target.unchecked_into();

                    let _ = element.set_pointer_capture(ev.pointer_id());
                }

                let pos = Point {
                    x: ev.client_x() as f64,
                    y: ev.client_y() as f64,
                };

                pointer.set(pos);

                panning.set(true);

                world_origin.set(offset.get());
                pan_origin.set(pos);
            }
        };

        let on_pointer_move = move |ev: PointerEvent| {
            pointer.set(Point {
                x: ev.client_x() as f64,
                y: ev.client_y() as f64,
            });
        };

        let stop_panning = move || {
            panning.set(false);
        };

        let on_pointer_up = move |ev: PointerEvent| {
            if ev.button() == 1 {
                stop_panning();
            }
        };

        let on_pointer_cancel = move |_| {
            stop_panning();
        };

        Effect::new(move |_| {
            if !panning.get() {
                return;
            }

            let origin = world_origin.get();
            let anchor = pan_origin.get();
            let current = pointer.get();

            offset.set(Point {
                x: origin.x + (current.x - anchor.x),
                y: origin.y + (current.y - anchor.y),
            });
        });

        view! {
            <div
                class="canvas_overlay"
                on:wheel=on_wheel
                on:pointerdown=on_pointer_down
                on:pointermove=on_pointer_move
                on:pointerup=on_pointer_up
                on:pointercancel=on_pointer_cancel
            />

            <div
                on:wheel=on_wheel
                on:pointerdown=on_pointer_down
                on:pointermove=on_pointer_move
                on:pointerup=on_pointer_up
                on:pointercancel=on_pointer_cancel
                style:position="absolute"
                style:transform=move || {
                    format!(
                        "translate({:.2}px, {:.2}px) scale({:.4})",
                        offset.get().x,
                        offset.get().y,
                        scale.get(),
                    )
                }
                style:transform-origin="0 0"
                style:will-change="transform"
            >
                {children()}
            </div>
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        view! {
            <div class="canvas_overlay" />
            <div style:position="absolute">
                {children()}
            </div>
        }
    }
}
