use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[component]
pub fn Table(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table", class);

    rsx! {
        table {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableHeader(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-header", class);

    rsx! {
        thead {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableBody(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-body", class);

    rsx! {
        tbody {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableFooter(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-footer", class);

    rsx! {
        tfoot {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableRow(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-row", class);

    rsx! {
        tr {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableHead(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-head", class);

    rsx! {
        th {
            class: classes,
            scope: "col",
            {children}
        }
    }
}

#[component]
pub fn TableCell(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-cell", class);

    rsx! {
        td {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableCaption(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-caption", class);

    rsx! {
        caption {
            class: classes,
            {children}
        }
    }
}
