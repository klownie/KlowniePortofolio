use std::fs;

use crate::app::HoveredNodeContext;
use leptos::{attr::Imagesrcset, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Canvas {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum Node {
    #[serde(rename = "group")]
    Group {
        id: String,
        x: isize,
        y: isize,
        width: isize,
        height: isize,
        color: String,
        label: String,
    },

    #[serde(rename = "file")]
    File {
        id: String,
        file: String,
        x: isize,
        y: isize,
        width: isize,
        height: isize,
    },

    #[serde(rename = "text")]
    Text {
        id: String,
        text: String,
        #[serde(default)]
        #[serde(rename = "styleAttributes")]
        style_attributes: Option<StyleAttributes>,
        x: isize,
        y: isize,
        width: isize,
        height: isize,
    },
    #[serde(rename = "link")]
    Link {
        id: String,
        url: String,
        x: isize,
        y: isize,
        width: isize,
        height: isize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StyleAttributes {
    #[serde(rename = "textAlign")]
    text_align: Option<String>,
}

#[server]
pub(crate) async fn load_canvas(name: String) -> Result<Canvas, ServerFnError> {
    Ok(parse_canvas(&format!("canvas/{name}.canvas")))
}

pub(crate) fn parse_canvas(path: &str) -> Canvas {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

pub(crate) fn render_node(node: Node) -> AnyView {
    match node {
        Node::File {
            file,
            x,
            y,
            width,
            height,
            ..
        } => {
            match std::path::Path::new(&file)
                .extension()
                .and_then(|e| e.to_str())
            {
                Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "avif") => view! {
                    <ImageNode file x y width height />
                }
                .into_any(),

                Some("md") => view! {
                    <MarkDownNode file x y width height />
                }
                .into_any(),

                _ => view! {
                    <div>"Unsupported File Type"</div>
                }
                .into_any(),
            }
        }

        Node::Text {
            text,
            x,
            y,
            width,
            height,
            style_attributes: styleAttributes,
            ..
        } => view! {
            <TextNode
                text
                x
                y
                width
                height
                text_align=styleAttributes
                    .and_then(|s| s.text_align)
                    .unwrap_or_else(|| "left".into())
            />
        }
        .into_any(),

        Node::Group {
            label,
            color,
            x,
            y,
            width,
            height,
            ..
        } => view! {
            <GroupNode label color x y width height />
        }
        .into_any(),

        Node::Link {
            url,
            x,
            y,
            width,
            height,
            ..
        } => view! {
            <LinkNode url x y width height />
        }
        .into_any(),
    }
}

#[component]
pub fn ImageNode(file: String, x: isize, y: isize, width: isize, height: isize) -> impl IntoView {
    let hover = expect_context::<HoveredNodeContext>();

    let on_hover = {
        let file = file.clone();

        move |_| {
            if hover.fullscreen.get() {
                return;
            }

            hover.file.set(Some(file.clone()));
            hover.position.set(Some((x, y)));
            hover.size.set(Some((width, height)));
        }
    };

    let on_leave = {
        move |_| {
            if hover.fullscreen.get() {
                return;
            }

            hover.file.set(None);
        }
    };

    let on_click = {
        move |_| {
            hover.fullscreen.update(|f| *f = !*f);
        }
    };

    view! {
        <img
            class:focused=move || hover.fullscreen.get()
            on:mouseenter=on_hover
            on:mouseleave=on_leave
            on:click=on_click
            loading="lazy"
            src=file
            style=format!(
                "position:absolute;\
                 left:{}px;\
                 top:{}px;\
                 width:{}px;\
                 height:{}px;\
                 object-fit:cover;",
                x, y, width, height
            )
        />
    }
}

#[component]
pub fn MarkDownNode(
    file: String,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
) -> impl IntoView {
    let content = fs::read_to_string(&file).unwrap_or("Could not load file".into());
    view! {
        <p
            style=format!(
                "position:absolute;\
                 left:{}px;\
                 top:{}px;\
                 width:{}px;\
                 height:{}px;\
                 object-fit:cover;",
                x, y, width, height
            )
        >
            {content}
        </p>
    }
}

#[component]
pub fn TextNode(
    text: String,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    text_align: String,
) -> impl IntoView {
    view! {
        <p
            style=format!(
                "position:absolute;\
                 left:{}px;\
                 top:{}px;\
                 width:{}px;\
                 height:{}px;\
                 margin:0;\
                 text-align:{};",
                x, y, width, height, text_align
            )
        >
            {text}
        </p>
    }
}

#[component]
pub fn GroupNode(
    label: String,
    color: String,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
) -> impl IntoView {
    view! {
        <section
            style=format!(
                "position:absolute;\
                 left:{}px;\
                 top:{}px;\
                 width:{}px;\
                 height:{}px;\
                 outline: max(calc(2px / var(--scale)), 5px) solid hsl(from {} h s l / 50%);\
                 background-color:hsl(from {} h s l / 5%);",
                x, y, width, height, color, color
            )
        >
            <label style=format!("background-color:{};", color)>
                {label}
            </label>
        </section>
    }
}

#[component]
pub fn LinkNode(url: String, x: isize, y: isize, width: isize, height: isize) -> impl IntoView {
    view! {
        <iframe
            src=url
            style=format!(
                "position:absolute;\
                 left:{}px;\
                 top:{}px;\
                 width:{}px;\
                 height:{}px;",
                x, y, width, height
            )
        />
    }
}
