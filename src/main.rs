use std::path::{Path, PathBuf};

use image::ImageReader;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use portfolio_website::app::*;

    // resize_canvas_images();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
#[cfg(not(feature = "resize"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

#[cfg(feature = "resize")]
fn main() {
  resize_canvas_images();
}

fn resize_canvas_images() {
    let pattern = "public/canvas/Canvas/assets/*/*";

    for entry in glob::glob(pattern).expect("Invalid glob pattern") {
        let path = match entry {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to read directory entry: {e}");
                continue;
            }
        };

        // Ignore directories
        if !path.is_file() {
            continue;
        }

        // Get extension
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase());

        // Only consider image extensions we care about
        match extension.as_deref() {
            Some("jpg") | Some("jpeg") | Some("png") | Some("webp") => {}
            _ => continue,
        }

        // Don't process files that are already resized
        if path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .is_some_and(|stem| stem.ends_with("_resized"))
        {
            continue;
        }

        // Output path
        let output = resized_path(&path);

        // Already generated → skip
        if output.exists() {
            println!("Already exists, skipping {:?}", output);
            continue;
        }

        println!("Processing {:?}", path);

        // Open image
        let reader = match ImageReader::open(&path) {
            Ok(reader) => reader,

            Err(e) => {
                eprintln!("Skipping {:?}: failed to open: {}", path, e);
                continue;
            }
        };

        // Detect the actual format from the file contents
        let reader = match reader.with_guessed_format() {
            Ok(reader) => reader,

            Err(e) => {
                eprintln!(
                    "Skipping {:?}: couldn't determine image format: {}",
                    path, e
                );
                continue;
            }
        };

        // Decode
        let img = match reader.decode() {
            Ok(img) => img,

            Err(e) => {
                eprintln!("Skipping {:?}: failed to decode image: {}", path, e);
                continue;
            }
        };

        let max_side = img.width().max(img.height());

        let (width, height) = if max_side >= 480 {
            let scale = 479.0 / max_side as f32;
            (
                (img.width() as f32 * scale) as u32,
                (img.height() as f32 * scale) as u32,
            )
        } else {
            (img.width(), img.height())
        };

        let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

        // Save as AVIF
        if let Err(e) = resized.save_with_format(&output, image::ImageFormat::Avif) {
            eprintln!("Failed to save AVIF {:?}: {}", output, e);

            continue;
        }

        println!("Created {:?}", output);
    }
}

fn resized_path(path: &Path) -> PathBuf {
    let stem = path.file_stem().unwrap().to_string_lossy();

    path.with_file_name(format!("{stem}_small.avif"))
}
