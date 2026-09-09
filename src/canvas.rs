use std::fs;

use crate::app::NodeContext;
use leptos::{attr::Imagesrcset, html::Hgroup, prelude::*};
use leptos_md::Markdown;
use leptos_use::{UseElementBoundingReturn, use_element_bounding};
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
pub(crate) struct StyleAttributes {
    #[serde(rename = "textAlign")]
    pub text_align: Option<String>,
}

#[server]
pub(crate) async fn load_canvas(name: String) -> Result<Canvas, ServerFnError> {
    let path = format!("public/canvas/Canvas/{name}.canvas");
    let contents = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&contents)?)
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

                Some("webm") => view! {
                    <VideoNode file x y width height />
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
pub fn ImageNode(
    file: String,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
) -> impl IntoView {
    let node_context = expect_context::<NodeContext>();
    let small_file = {
        let path = std::path::Path::new(&file);

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        path.with_file_name(format!("{stem}_small.avif"))
            .to_string_lossy()
            .into_owned()
    };

    let source = RwSignal::new(small_file);

    let on_mouse_enter = {
        let value = file.clone();
        move |_| {
            source.set(value.clone());
        }
    };

    let on_click = {
        let file = file.clone();

        move |_| {
            if node_context.fullscreen.get() {
                return;
            }

            node_context.file.set(Some(file.clone()));
            node_context.position.set(Some((x, y)));
            node_context.size.set(Some((width, height)));
        }
    };

    view! {
        <img
            class:focused=move || node_context.fullscreen.get()
            on:click=on_click
            on:mouseenter=on_mouse_enter
            loading="lazy"
            src=source
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
pub fn VideoNode(file: String, x: isize, y: isize, width: isize, height: isize) -> impl IntoView {
    view! {
        <video autoplay loop muted playsinline
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
          <source src={file} type="video/webm" />
        </video>
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
        <hgroup
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
            <Markdown content={text} />
        </hgroup>
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
