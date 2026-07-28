use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_params_map,
    path,
};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Canvas {
    nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Node {
    #[serde(rename = "group")]
    Group {
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: String,
        label: String,
    },

    #[serde(rename = "file")]
    File {
        id: String,
        file: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },

    #[serde(rename = "text")]
    Text {
        id: String,
        text: String,
        #[serde(default)]
        styleAttributes: Option<StyleAttributes>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    #[serde(rename = "link")]
    Link {
        id: String,
        url: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
}

#[derive(Debug, Deserialize)]
struct StyleAttributes {
    textAlign: Option<String>,
}

#[server]
pub async fn load_canvas(name: String) -> Result<String, ServerFnError> {
    Ok(parse_canvas(&format!("canvas/{name}.canvas")))
}

fn parse_canvas(path: &str) -> String {
    let contents = fs::read_to_string(path).unwrap();

    let canvas: Canvas = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let mut html = String::new();

    for node in &canvas.nodes {
        match node {
            Node::Group {
                color,
                label,
                x,
                y,
                width,
                height,
                ..
            } => {
                html.push_str(&format!(
                    r#"
<section
    style="
        position:absolute;
        left:{}px;
        top:{}px;
        width:{}px;
        height:{}px;
        border:5px solid hsl(from {} h s l / 50%);
        background-color: hsl(from {} h s l / 5%);;
        box-sizing:border-box;
        padding:8px;
    "
>
    <label style="background-color: {};">{}</label>
</section>
"#,
                    x, y, width, height, color, color, color, label
                ));
            }

            Node::File {
                file,
                x,
                y,
                width,
                height,
                ..
            } => {
                let html_node = match std::path::Path::new(file)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    Some("md") => {
                        let content = fs::read_to_string(file)
                            .unwrap_or_else(|_| format!("Failed to load {}", file));

                        format!(
                            r#"
<div
    style="
        position:absolute;
        left:{}px;
        top:{}px;
        width:{}px;
        height:{}px;
        overflow:auto;
        box-sizing:border-box;
        padding:10px;
    "
>
    {}
</div>
"#,
                            x, y, width, height, content
                        )
                    }

                    Some("png" | "jpg" | "jpeg" | "webp" | "gif") => {
                        format!(
                            r#"
<img
    src="{}"
    style="
        position:absolute;
        left:{}px;
        top:{}px;
        width:{}px;
        height:{}px;
        object-fit:cover;
    "
/>
"#,
                            file, x, y, width, height
                        )
                    }

                    Some(ext) => {
                        format!(
                            r#"
<div
    style="
        position:absolute;
        left:{}px;
        top:{}px;
        width:{}px;
        height:{}px;
        border:1px solid gray;
        padding:10px;
        box-sizing:border-box;
    "
>
    Unsupported file: {}
</div>
"#,
                            x, y, width, height, ext
                        )
                    }

                    None => {
                        format!(
                            r#"
<div
    style="
        position:absolute;
        left:{}px;
        top:{}px;
        width:{}px;
        height:{}px;
    "
>
    {}
</div>
"#,
                            x, y, width, height, file
                        )
                    }
                };

                html.push_str(&html_node);
            }

            Node::Text {
                text,
                x,
                y,
                width,
                height,
                styleAttributes,
                ..
            } => {
                html.push_str(&format!(
                    r#"
<p
    style="
        position:absolute;
        left:{}px;
        top:{}px;
        width:{}px;
        height:{}px;
        margin:0;
        justify-content: {};
    "
>
    {}
</p>
"#,
                    x,
                    y,
                    width,
                    height,
                    styleAttributes
                        .as_ref()
                        .and_then(|s| s.textAlign.as_deref())
                        .unwrap_or("left"),
                    text
                ));
            }
            Node::Link {
                url,
                x,
                y,
                width,
                height,
                ..
            } => {
                html.push_str(&format!(
                    r#"
<iframe
    src="{}"
    style="
        position:absolute;
        left:{}px;
        top:{}px;
        width:{}px;
        height:{}px;
        margin:0;
    "

></iframe>
"#,
                    url, x, y, width, height,
                ));
            }
        }
    }

    html
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio_website.css" />

        <Title text="Audrick Yeu | Portfolio" />

        <Router>
            <main>
                <SideBar />

                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/canvas/:name") view=ObsidianCanvas />
                </Routes>
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
        |name| async move { load_canvas(name).await.unwrap_or_default() },
    );

    view! {
        <Suspense fallback=|| view! {
            <div class="loading">"Loading canvas..."</div>
        }>
            {move || {
                canvas.get().map(|canvas_file| {
                    view! {
                        <MouseTracker>
                            <div
                                class="obsidian_canvas"
                                inner_html=canvas_file
                            />
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
fn SideBar() -> impl IntoView {
    use leptos_router::components::A;

    let canvases = Resource::new(
        || (),
        |_| async { list_canvases().await.unwrap_or_default() },
    );

    view! {
        <nav class="sidebar">
            <Suspense fallback=|| view! {
                <ul>
                    <li>"Loading…"</li>
                </ul>
            }>
                {move || {
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
                }}
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

            let new_scale = (old_scale * zoom_factor).clamp(0.1, 10.0);

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
