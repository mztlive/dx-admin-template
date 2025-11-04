use dioxus::html::events::{DragEvent, FormEvent};
use dioxus::html::{FileData, HasFileData};
use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub content_type: Option<String>,
}

fn collect_metadata(files: Vec<FileData>) -> Vec<FileMetadata> {
    files
        .into_iter()
        .map(|file| FileMetadata {
            name: file.name(),
            size: file.size(),
            content_type: file.content_type(),
        })
        .collect()
}

#[component]
pub fn FileDropZone(
    #[props(into, default)] class: Option<String>,
    #[props(default)] multiple: bool,
    #[props(into, default)] accept: Option<String>,
    #[props(optional)] on_files: Option<EventHandler<Vec<FileMetadata>>>,
    #[props(optional)] content: Option<Element>,
) -> Element {
    let classes = merge_class("ui-dropzone", class);
    let accept_attr = accept.unwrap_or_default();
    let is_active = use_signal(|| false);
    let selected_files = use_signal(|| Vec::<FileMetadata>::new());
    let on_files_handler = on_files.clone();

    let upload_summary = move || {
        let files = selected_files();
        if files.is_empty() {
            "No files selected yet".to_string()
        } else if files.len() == 1 {
            format!(
                "Ready to upload “{}” ({:.1} KB)",
                files[0].name,
                files[0].size as f64 / 1024.0
            )
        } else {
            format!(
                "{} files queued • total {:.1} KB",
                files.len(),
                files.iter().map(|f| f.size as f64).sum::<f64>() / 1024.0
            )
        }
    };

    let body_content: Element = content.unwrap_or_else(|| {
        rsx! {
            div {
                class: "ui-stack",
                span { class: "ui-dropzone-title", "Drag & drop files" }
                span { class: "ui-field-helper", "or click to browse from your computer" }
            }
        }
    });

    rsx! {
        label {
            class: classes,
            "data-state": if is_active() { "active" } else { "idle" },
            ondragenter: {
                let mut hovering = is_active.clone();
                move |event: DragEvent| {
                    event.prevent_default();
                    hovering.set(true);
                }
            },
            ondragover: {
                let mut hovering = is_active.clone();
                move |event: DragEvent| {
                    event.prevent_default();
                    hovering.set(true);
                }
            },
            ondragleave: {
                let mut hovering = is_active.clone();
                move |_event: DragEvent| hovering.set(false)
            },
            ondrop: {
                let mut hovering = is_active.clone();
                let mut selected = selected_files.clone();
                let handler = on_files_handler.clone();
                move |event: DragEvent| {
                    event.prevent_default();
                    hovering.set(false);
                    let files = collect_metadata(event.data().files());
                    selected.set(files.clone());
                    if let Some(callback) = handler.clone() {
                        callback.call(files);
                    }
                }
            },
            input {
                class: "ui-dropzone-input",
                r#type: "file",
                multiple,
                accept: accept_attr.clone(),
                onchange: {
                    let mut selected = selected_files.clone();
                    let handler = on_files_handler.clone();
                    move |event: FormEvent| {
                        let files = collect_metadata(event.files());
                        selected.set(files.clone());
                        if let Some(callback) = handler.clone() {
                            callback.call(files);
                        }
                    }
                },
            }
            div {
                class: "ui-dropzone-body",
                {body_content}
                div {
                    class: "ui-dropzone-summary",
                    "{upload_summary()}"
                }
            }
        }
    }
}
