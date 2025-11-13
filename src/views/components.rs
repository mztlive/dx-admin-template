use crate::components::ui::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, Alert, AlertVariant, AspectRatio,
    Avatar, Badge, BadgeVariant, Breadcrumb, Button, ButtonSize, ButtonVariant, Calendar, Card,
    CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Checkbox, Collapsible,
    CollapsibleContent, CollapsibleTrigger, Combobox, ComboboxOption, CommandItem, CommandPalette,
    ContextItem, ContextMenu, Crumb, DateRange, DateRangePicker, Dialog, DropdownMenu,
    DropdownMenuItem, FileDropZone, FileMetadata, FormField, FormMessage, FormMessageVariant,
    HoverCard, Input, Label, Menubar, MenubarItem, MenubarMenu, NavigationItem, NavigationMenu,
    Pagination, Popover, Progress, RadioGroup, RadioGroupItem, ScrollArea, Select, SelectOption,
    Separator, SeparatorOrientation, Sheet, SheetSide, Sidebar, SidebarContent, SidebarFooter,
    SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset,
    SidebarLayout, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarSeparator,
    SidebarTrigger, Skeleton, Slider, StepItem, Steps, Switch, Table, TableBody, TableCaption,
    TableCell, TableFooter, TableHead, TableHeader, TableRow, Tabs, TabsContent, TabsList,
    TabsTrigger, Textarea, Toast, ToastViewport, Toggle, ToggleGroup, ToggleGroupItem,
    ToggleGroupMode, ToggleGroupOrientation, Tooltip,
};
use crate::time::NaiveDate;
use dioxus::html::events::FormEvent;
use dioxus::prelude::*;

#[component]
pub fn Components() -> Element {
    rsx! {
        div {
            class: "component-page",
            div {
                class: "page-heading",
                h1 { "Component library" }
                p { "Browse every primitive wired into this starter so new screens stay consistent." }
            }
            UiShowcase {}
        }
    }
}
#[component]
fn UiShowcase() -> Element {
    let accepted_terms = use_signal(|| false);
    let email_notifications = use_signal(|| true);
    let slider_value = use_signal(|| 42.0f32);
    let contact_method = use_signal(|| "email".to_string());
    let newsletter_opt_in = use_signal(|| true);
    let dark_mode = use_signal(|| false);
    let theme_choice = use_signal(|| Some("system".to_string()));
    let menu_selection = use_signal(|| "Select a menu action".to_string());
    let menubar_selection = use_signal(|| "Choose a menu item".to_string());
    let pagination_current = use_signal(|| 3usize);
    let steps_current = use_signal(|| 2usize);
    let command_selection = use_signal(|| "Nothing selected yet".to_string());
    let context_selection = use_signal(|| "Right click the area to choose an action".to_string());
    let dialog_open = use_signal(|| false);
    let sheet_open = use_signal(|| false);
    let toast_open = use_signal(|| false);
    let sidebar_collapsed = use_signal(|| false);
    let sidebar_active = use_signal(|| "analytics".to_string());
    let profile_name = use_signal(|| "".to_string());
    let name_error =
        use_signal(|| Some("Use at least 3 characters to stay descriptive.".to_string()));
    let combobox_selection = use_signal(|| Some("analytics".to_string()));
    let toggle_active = use_signal(|| true);
    let calendar_selection =
        use_signal(|| Some(NaiveDate::from_ymd_opt(2024, 6, 11).expect("valid date")));
    let collapsible_open = use_signal(|| false);
    let toggle_group_values = use_signal(|| vec!["daily".to_string()]);
    let date_range_value = use_signal(|| {
        Some(DateRange::new(
            NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid start"),
            NaiveDate::from_ymd_opt(2024, 6, 7).expect("valid end"),
        ))
    });
    let dropzone_files = use_signal(|| Vec::<FileMetadata>::new());
    let slider_value_signal = slider_value.clone();
    let slider_value_setter = slider_value.clone();
    let contact_method_signal = contact_method.clone();
    let theme_choice_signal = theme_choice.clone();
    let accepted_terms_setter = accepted_terms.clone();
    let email_notifications_setter = email_notifications.clone();
    let contact_method_setter = contact_method.clone();
    let newsletter_opt_in_setter = newsletter_opt_in.clone();
    let dark_mode_setter = dark_mode.clone();
    let theme_choice_setter = theme_choice.clone();
    let menu_selection_setter = menu_selection.clone();
    let menubar_selection_setter = menubar_selection.clone();
    let pagination_setter = pagination_current.clone();
    let steps_setter = steps_current.clone();
    let command_selection_setter = command_selection.clone();
    let context_selection_setter = context_selection.clone();
    let dialog_signal = dialog_open.clone();
    let sheet_signal = sheet_open.clone();
    let toast_signal = toast_open.clone();
    let sidebar_collapsed_setter = sidebar_collapsed.clone();
    let sidebar_active_setter = sidebar_active.clone();
    let profile_name_signal = profile_name.clone();
    let profile_name_setter = profile_name.clone();
    let name_error_signal = name_error.clone();
    let name_error_setter = name_error.clone();
    let combobox_selection_signal = combobox_selection.clone();
    let combobox_selection_setter = combobox_selection.clone();
    let toggle_active_signal = toggle_active.clone();
    let toggle_active_setter = toggle_active.clone();
    let calendar_selection_signal = calendar_selection.clone();
    let calendar_selection_setter = calendar_selection.clone();
    let collapsible_signal = collapsible_open.clone();
    let collapsible_setter = collapsible_open.clone();
    let toggle_group_signal = toggle_group_values.clone();
    let toggle_group_setter = toggle_group_values.clone();
    let date_range_signal = date_range_value.clone();
    let date_range_setter = date_range_value.clone();
    let dropzone_files_signal = dropzone_files.clone();
    let dropzone_files_setter = dropzone_files.clone();
    let intensity_text = move || format!("Accent intensity: {:.0}%", slider_value_signal());
    let contact_text = move || format!("Preferred contact: {}", contact_method_signal());
    let profile_preview = move || {
        let value = profile_name_signal();
        if value.trim().is_empty() {
            "Name is currently empty".to_string()
        } else {
            format!("Display name preview: {}", value.trim())
        }
    };
    let combobox_summary = move || {
        if let Some(value) = combobox_selection_signal() {
            format!("Project owner: {value}")
        } else {
            "Assign a project owner to sync permissions.".to_string()
        }
    };
    let calendar_summary = move || {
        if let Some(date) = calendar_selection_signal() {
            format!("Next milestone: {}", date.format("%b %d, %Y"))
        } else {
            "Pick a date to keep the timeline on track.".to_string()
        }
    };
    let toggle_summary = move || {
        if toggle_active_signal() {
            "Emails are enabled for this workflow.".to_string()
        } else {
            "Emails are paused until you re-enable them.".to_string()
        }
    };
    let toggle_group_summary = move || {
        let values = toggle_group_signal();
        if values.is_empty() {
            "No frequencies selected.".to_string()
        } else {
            format!("Cadence: {}", values.join(", "))
        }
    };
    let range_preview = move || match date_range_signal() {
        Some(range) if range.start != range.end => format!(
            "Range: {} → {}",
            range.start.format("%b %d"),
            range.end.format("%b %d %Y")
        ),
        Some(range) => format!("Single day: {}", range.start.format("%b %d, %Y")),
        None => "Pick a date window to compare analytics.".to_string(),
    };
    let dropzone_summary = move || {
        let files = dropzone_files_signal();
        if files.is_empty() {
            "No assets queued.".to_string()
        } else {
            format!("{} file(s) staged.", files.len())
        }
    };
    let select_options = vec![
        SelectOption::new("System", "system"),
        SelectOption::new("Light", "light"),
        SelectOption::new("Dark", "dark"),
    ];
    let calendar_month = NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date");
    let combobox_options = vec![
        ComboboxOption::new("Analytics", "analytics")
            .with_description("Dashboards, funnels, and trend alerts"),
        ComboboxOption::new("Growth", "growth")
            .with_description("Lifecycle campaigns and experiments"),
        ComboboxOption::new("Infrastructure", "infrastructure")
            .with_description("Runtime, deploys, and observability"),
        ComboboxOption::new("Support", "support")
            .with_description("Queues, macros, and response goals"),
    ];
    let menu_items = vec![
        DropdownMenuItem::new("Profile", "profile").with_shortcut("⌘P"),
        DropdownMenuItem::new("Billing", "billing").with_shortcut("⌘B"),
        DropdownMenuItem::new("Team", "team"),
        DropdownMenuItem::new("Sign out", "logout").destructive(),
    ];
    let breadcrumb_items = vec![
        Crumb::new("Dashboard", Some("#")),
        Crumb::new("Settings", Some("#settings")),
        Crumb::new("Team", None::<String>),
    ];
    let navigation_items = vec![
        NavigationItem::new(
            "Overview",
            "#overview",
            Some("Project snapshots and quick metrics"),
        ),
        NavigationItem::new(
            "Playground",
            "#playground",
            Some("Prototype new ideas and components"),
        ),
        NavigationItem::new(
            "Documentation",
            "https://dioxuslabs.com/learn",
            Some("Dive into the latest Dioxus 0.7 docs"),
        ),
    ];
    let table_rows = vec![
        ("DW-9021", "Realtime dashboard", "Shipping", "2 minutes ago"),
        (
            "DB-1740",
            "AI campaign assistant",
            "Review",
            "14 minutes ago",
        ),
        ("MR-1183", "Metrics service", "Building", "38 minutes ago"),
        ("PK-9422", "Payments ledger", "Queued", "58 minutes ago"),
        ("XD-7710", "Access gateway", "Paused", "2 hours ago"),
    ];
    let activity_items = vec![
        ("09:05", "Jesse", "merged \"navigation cleanups\" into main"),
        ("10:18", "Mia", "scheduled the weekly metrics export"),
        ("11:42", "Arjun", "paused the experiment \"Pricing v2\""),
        ("12:03", "Ivy", "restarted the realtime analytics workers"),
        ("12:44", "Kai", "commented on the onboarding funnel deck"),
        ("13:27", "Lena", "acknowledged alert \"Queue depth\""),
    ];
    let menubar_menus = vec![
        MenubarMenu::new(
            "File",
            vec![
                MenubarItem::new("New Tab", "new_tab").shortcut("⌘T"),
                MenubarItem::new("Open Workspace", "open_workspace"),
                MenubarItem::new("Save", "save").shortcut("⌘S"),
            ],
        ),
        MenubarMenu::new(
            "Edit",
            vec![
                MenubarItem::new("Undo", "undo").shortcut("⌘Z"),
                MenubarItem::new("Redo", "redo").shortcut("⇧⌘Z"),
                MenubarItem::new("Delete", "delete").destructive(),
            ],
        ),
    ];
    let command_items = vec![
        CommandItem::new("Create project", "create_project")
            .shortcut("⌘N")
            .group("Actions"),
        CommandItem::new("Invite teammate", "invite").group("Actions"),
        CommandItem::new("Open documentation", "docs").group("Resources"),
        CommandItem::new("Keyboard shortcuts", "shortcuts").group("Resources"),
    ];
    let context_items = vec![
        ContextItem::new("Rename", "rename"),
        ContextItem::new("Duplicate", "duplicate"),
        ContextItem::new("Archive", "archive"),
        ContextItem::new("Delete", "delete").destructive(),
    ];
    let steps_items = vec![
        StepItem::new("Plan", Some("Outline requirements")),
        StepItem::new("Build", Some("Implement features")),
        StepItem::new("Review", Some("QA and ship")),
    ];
    let total_pages = 8usize;
    let pagination_summary =
        move || format!("Showing page {} of {total_pages}", pagination_current());
    let steps_total = steps_items.len();
    let steps_summary = move || format!("Stage {} of {steps_total}", steps_current());
    let theme_display = {
        let current = theme_choice();
        current
            .as_ref()
            .and_then(|value| {
                select_options
                    .iter()
                    .find(|option| option.value == *value)
                    .map(|option| option.label.clone())
            })
            .unwrap_or_else(|| "System".to_string())
    };
    let theme_summary = format!("Active theme: {theme_display}");
    let collapsed_state = sidebar_collapsed();
    let current_sidebar_value = sidebar_active();
    let is_analytics_active = current_sidebar_value.as_str() == "analytics";
    let is_crm_active = current_sidebar_value.as_str() == "crm";
    let is_billing_active = current_sidebar_value.as_str() == "billing";
    let is_settings_active = current_sidebar_value.as_str() == "settings";
    let (sidebar_title, sidebar_body) = match current_sidebar_value.as_str() {
        "analytics" => (
            "Analytics overview".to_string(),
            "Monitor KPI trends, conversion funnels, and health metrics in real time.".to_string(),
        ),
        "crm" => (
            "Customer relationship management".to_string(),
            "Surface leads, segment accounts, and coordinate follow-ups in one place.".to_string(),
        ),
        "billing" => (
            "Billing & usage".to_string(),
            "Review invoices, adjust subscription tiers, and reconcile metered usage.".to_string(),
        ),
        "settings" => (
            "Workspace settings".to_string(),
            "Manage authentication, API tokens, and notification preferences.".to_string(),
        ),
        _ => (
            "Select a section".to_string(),
            "Pick a destination from the sidebar to preview the content area.".to_string(),
        ),
    };
    let scroll_container_style = "width: 100%; overflow-x: auto; padding-bottom: 2.5rem;";
    let showcase_grid_style = "display: grid; grid-template-columns: repeat(2, minmax(520px, 1fr)); gap: 2.5rem; align-items: start; min-width: 1100px;";
    let full_width_style = "grid-column: 1 / -1; min-width: 520px;";
    let single_column_style = "min-width: 520px;";

    rsx! {
        section {
            class: if dark_mode() { "ui-shell shadcn dark" } else { "ui-shell shadcn" },
            "data-theme": if dark_mode() { "dark" } else { "light" },

            div {
                class: "ui-showcase-scroll",
                style: scroll_container_style,
                div {
                    class: "ui-showcase-grid",
                    style: showcase_grid_style,

                    div {
                        style: full_width_style,
                        Card {
                            CardHeader {
                                CardTitle { "Profile form" }
                                CardDescription { "Inputs, sliders, helpers, and actions inside a card layout." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    Label { html_for: "profile-name", "Name" }
                                    Input { id: "profile-name", placeholder: "Ada Lovelace" }
                                }
                                div { class: "ui-stack",
                                    Label { html_for: "profile-about", "About" }
                                    Textarea {
                                        id: "profile-about",
                                        placeholder: "Tell us something fun...",
                                        rows: 4,
                                    }
                                    SpanHelper { "Textarea adopts shadcn spacing and typography out of the box." }
                                }
                                Separator { style: "margin: 1rem 0;" }
                                div { class: "ui-stack",
                                    Label { html_for: "accent-slider", "Accent strength" }
                                    Slider {
                                        value: slider_value(),
                                        min: 0.0,
                                        max: 100.0,
                                        step: 1.0,
                                        on_value_change: {
                                            let mut signal = slider_value_setter.clone();
                                            move |val| signal.set(val)
                                        },
                                    }
                                    Progress { value: slider_value(), max: 100.0 }
                                    SpanHelper { "{intensity_text()}" }
                                }
                                div { class: "ui-stack",
                                    Label { html_for: "theme-select", "Theme preference" }
                                    Select {
                                        id: Some("theme-select".to_string()),
                                        placeholder: "Select a theme",
                                        options: select_options.clone(),
                                        selected: theme_choice_signal(),
                                        on_change: move |value| {
                                            let mut signal = theme_choice_setter.clone();
                                            signal.set(Some(value));
                                        },
                                    }
                                    SpanHelper { "{theme_summary}" }
                                }
                                div { class: "ui-bleed",
                                        div { class: "ui-cluster",
                                            Checkbox {
                                                id: Some("accept-terms".to_string()),
                                                checked: accepted_terms(),
                                                on_checked_change: move |state| accepted_terms_setter.clone().set(state),
                                            }
                                            Label { html_for: "accept-terms", "Agree to terms" }
                                        }
                                        div { class: "ui-cluster",
                                            Label { html_for: "profile-emails", "Email notifications" }
                                            Switch {
                                                id: Some("profile-emails".to_string()),
                                                checked: email_notifications(),
                                                on_checked_change: move |state| email_notifications_setter.clone().set(state),
                                            }
                                        }
                                    }
                            }
                            CardFooter {
                                div { class: "ui-cluster",
                                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Cancel" }
                                    Button { disabled: !accepted_terms(), "Save changes" }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Form helpers" }
                                CardDescription { "FormField, Combobox, and Toggle wire up validation with shadcn styling." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    FormField {
                                        id: Some("helper-name".to_string()),
                                        label: Some("Project name".to_string()),
                                        helper_text: Some(profile_preview()),
                                        error: Some(name_error_signal),
                                        Input {
                                            id: "helper-name",
                                            placeholder: "Launch analytics workspace",
                                            value: profile_name_signal(),
                                            on_input: {
                                                let mut value_signal = profile_name_setter.clone();
                                                let mut error_signal = name_error_setter.clone();
                                                move |event: FormEvent| {
                                                    let value = event.value();
                                                    let trimmed_len = value.trim().len();
                                                    value_signal.set(value.clone());
                                                    if trimmed_len >= 3 {
                                                        error_signal.set(None);
                                                    } else {
                                                        error_signal.set(Some("Use at least 3 characters to stay descriptive.".to_string()));
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    FormField {
                                        id: Some("helper-brief".to_string()),
                                        label: Some("Summary".to_string()),
                                        description: Some("Share quick context for the owners reviewing this request.".to_string()),
                                        helper_text: Some("You can mention teammates with @ and use Markdown formatting.".to_string()),
                                        Textarea {
                                            id: "helper-brief",
                                            placeholder: "Outline the goal, stakeholders, and success signal...",
                                            rows: 4,
                                        }
                                    }
                                    FormField {
                                        id: Some("owner-combobox".to_string()),
                                        label: Some("Assign owner".to_string()),
                                        description: Some("Search across teams to hand off this initiative.".to_string()),
                                        helper_text: Some(combobox_summary()),
                                        Combobox {
                                            id: Some("owner-combobox".to_string()),
                                            placeholder: "Search by team...",
                                            options: combobox_options.clone(),
                                            selected: combobox_selection_signal(),
                                            on_select: {
                                                let mut setter = combobox_selection_setter.clone();
                                                move |value| setter.set(Some(value))
                                            },
                                        }
                                    }
                                    div { class: "ui-stack",
                                        Toggle {
                                            pressed: toggle_active_signal(),
                                            on_pressed_change: {
                                                let mut setter = toggle_active_setter.clone();
                                                move |state| setter.set(state)
                                            },
                                            "Email alerts"
                                        }
                                        FormMessage {
                                            variant: FormMessageVariant::Helper,
                                            class: Some("ui-field-helper".to_string()),
                                            "{toggle_summary()}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Layout & uploads" }
                                CardDescription { "Aspect ratios and drag-and-drop staging." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    AspectRatio {
                                        ratio: 16.0 / 9.0,
                                        div {
                                            style: "width: 100%; height: 100%; background: radial-gradient(circle at 20% 20%, hsl(var(--primary) / 0.3), transparent); border-radius: calc(var(--radius) - 2px); display: flex; align-items: center; justify-content: center; font-size: 0.85rem; color: hsl(var(--muted-foreground));",
                                            "Video or hero media stays perfectly scaled."
                                        }
                                    }
                                    FileDropZone {
                                        multiple: true,
                                        on_files: {
                                            let mut setter = dropzone_files_setter.clone();
                                            move |files| setter.set(files)
                                        },
                                        content: Some(rsx! {
                                            div {
                                                class: "ui-stack",
                                                span { class: "ui-dropzone-title", "Drop brand assets" }
                                                span { class: "ui-field-helper", "Supports PNG, SVG, and MP4 up to 200 MB." }
                                            }
                                        }),
                                    }
                                    FormMessage {
                                        variant: FormMessageVariant::Helper,
                                        class: Some("ui-field-helper".to_string()),
                                        "{dropzone_summary()}"
                                    }
                                    if !dropzone_files_signal().is_empty() {
                                        {
                                            let files = dropzone_files_signal();
                                            rsx! {
                                                ul {
                                                    style: "font-size: 0.8rem; display: flex; flex-direction: column; gap: 0.35rem;",
                                                    for file in files {
                                                        {
                                                            let label = format!("{} ({:.1} KB)", file.name, file.size as f64 / 1024.0);
                                                            rsx! { li { "{label}" } }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Schedules & ranges" }
                                CardDescription { "Collapsible filters, toggle groups, and dual-month range picking." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    Collapsible {
                                        open: collapsible_signal.clone(),
                                        on_open_change: {
                                            let mut setter = collapsible_setter.clone();
                                            move |state| setter.set(state)
                                        },
                                        CollapsibleTrigger { "Advanced filters" }
                                        CollapsibleContent {
                                            SpanHelper { "Keep optional controls tucked away until needed." }
                                            SpanHelper { "Current state: " }
                                            SpanHelper { if collapsible_signal() { "Expanded" } else { "Collapsed" } }
                                        }
                                    }
                                    div { class: "ui-stack",
                                        span { class: "ui-field-helper", "Delivery cadence" }
                                        ToggleGroup {
                                            values: toggle_group_signal.clone(),
                                            mode: ToggleGroupMode::Multiple,
                                            orientation: ToggleGroupOrientation::Horizontal,
                                            on_value_change: {
                                                let mut setter = toggle_group_setter.clone();
                                                move |values| setter.set(values)
                                            },
                                            ToggleGroupItem { value: "daily".to_string(), "Daily" }
                                            ToggleGroupItem { value: "weekly".to_string(), "Weekly" }
                                            ToggleGroupItem { value: "monthly".to_string(), "Monthly" }
                                        }
                                        FormMessage {
                                            variant: FormMessageVariant::Helper,
                                            class: Some("ui-field-helper".to_string()),
                                            "{toggle_group_summary()}"
                                        }
                                    }
                                    DateRangePicker {
                                        value: date_range_signal.clone(),
                                        on_change: {
                                            let mut setter = date_range_setter.clone();
                                            move |range| setter.set(range)
                                        },
                                        initial_month: Some(NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid month")),
                                    }
                                    FormMessage {
                                        variant: FormMessageVariant::Helper,
                                        class: Some("ui-field-helper".to_string()),
                                        "{range_preview()}"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Buttons & badges" }
                                CardDescription { "Variant + size matrix copied directly from shadcn/ui." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    SpanHelper { "Buttons – variants" }
                                    div { class: "ui-cluster",
                                        Button { "Primary" }
                                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                                        Button { variant: ButtonVariant::Destructive, "Destructive" }
                                        Button { variant: ButtonVariant::Outline, "Outline" }
                                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                                        Button { variant: ButtonVariant::Link, "Learn more" }
                                    }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Buttons – sizes" }
                                    div { class: "ui-cluster",
                                        Button { size: ButtonSize::Sm, "Small" }
                                        Button { "Default" }
                                        Button { size: ButtonSize::Lg, "Large" }
                                        Button { size: ButtonSize::Icon, "★" }
                                    }
                                }
                                Separator { style: "margin: 1rem 0;" }
                                div { class: "ui-stack",
                                    SpanHelper { "Badges" }
                                    div { class: "ui-cluster",
                                        Badge { "Default" }
                                        Badge { variant: BadgeVariant::Secondary, "Secondary" }
                                        Badge { variant: BadgeVariant::Destructive, "Destructive" }
                                        Separator { orientation: SeparatorOrientation::Vertical, style: "height: 1.5rem;" }
                                        Badge { variant: BadgeVariant::Outline, "Outline" }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Select & dropdowns" }
                                CardDescription { "Select, dropdown menu, tooltip and dynamic feedback." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    Label { html_for: "quick-theme", "Quick theme" }
                                    Select {
                                        id: Some("quick-theme".to_string()),
                                        placeholder: "Choose theme",
                                        options: select_options.clone(),
                                        selected: theme_choice_signal(),
                                        on_change: move |value| {
                                            let mut signal = theme_choice_setter.clone();
                                            signal.set(Some(value));
                                        },
                                    }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Dropdown menu" }
                                    DropdownMenu {
                                        label: "Open menu",
                                        items: menu_items.clone(),
                                        on_select: move |value| {
                                            let mut signal = menu_selection_setter.clone();
                                            signal.set(format!("Selected action: {value}"));
                                        },
                                    }
                                    SpanHelper { "{menu_selection()}" }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Tooltip" }
                                    Tooltip {
                                        label: "Invite collaborators",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Sm,
                                            "Hover me"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: full_width_style,
                        Card {
                            CardHeader {
                                CardTitle { "Data timelines" }
                                CardDescription { "Tables, scroll areas, calendars, and skeleton loaders keep admin dashboards responsive." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    SpanHelper { "Deploys" }
                                    ScrollArea {
                                        max_height: Some("220px".to_string()),
                                        Table {
                                            TableCaption { "Latest updates from the delivery pipeline." }
                                            TableHeader {
                                                TableRow {
                                                    TableHead { "ID" }
                                                    TableHead { "Project" }
                                                    TableHead { "Status" }
                                                    TableHead { "Updated" }
                                                }
                                            }
                                            TableBody {
                                                for (id, name, status, updated) in table_rows.iter().copied() {
                                                    TableRow {
                                                        TableCell { "{id}" }
                                                        TableCell { "{name}" }
                                                        TableCell { "{status}" }
                                                        TableCell { "{updated}" }
                                                    }
                                                }
                                            }
                                            TableFooter {
                                                TableRow {
                                                    TableCell { "Total" }
                                                    TableCell { "{table_rows.len()} pipelines" }
                                                    TableCell { class: Some("ui-field-helper".to_string()), "Automated checks" }
                                                    TableCell { class: Some("ui-field-helper".to_string()), "Past hour" }
                                                }
                                            }
                                        }
                                    }
                                    FormMessage {
                                        variant: FormMessageVariant::Helper,
                                        class: Some("ui-field-helper".to_string()),
                                        "Keep automation quick by streaming the hottest rows into view."
                                    }
                                    Separator { style: "margin: 1rem 0;" }
                                    SpanHelper { "Activity feed" }
                                    ScrollArea {
                                        max_height: Some("140px".to_string()),
                                        ul {
                                            style: "display: flex; flex-direction: column; gap: 0.6rem; font-size: 0.85rem;",
                                            for (time, author, action) in activity_items.iter().copied() {
                                                li {
                                                    style: "display: flex; gap: 0.5rem; align-items: baseline;",
                                                    span { style: "font-weight: 600; font-variant-numeric: tabular-nums;", "{time}" }
                                                    span { style: "font-weight: 600;", "{author}" }
                                                    span { style: "color: hsl(var(--muted-foreground));", "{action}" }
                                                }
                                            }
                                        }
                                    }
                                    Separator { style: "margin: 1rem 0;" }
                                    SpanHelper { "{calendar_summary()}" }
                                    Calendar {
                                        initial_month: calendar_month,
                                        selected: calendar_selection_signal(),
                                        on_select: {
                                            let mut setter = calendar_selection_setter.clone();
                                            move |day| setter.set(Some(day))
                                        },
                                    }
                                    div { class: "ui-cluster",
                                        Skeleton { width: Some("160px".to_string()), height: Some("1rem".to_string()) }
                                        Skeleton { width: Some("120px".to_string()), height: Some("1rem".to_string()) }
                                        Skeleton { width: Some("200px".to_string()), height: Some("1rem".to_string()) }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: full_width_style,
                        Card {
                            CardHeader {
                                CardTitle { "Navigation patterns" }
                                CardDescription { "Breadcrumbs, menus, pagination, and progress steps." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    SpanHelper { "Breadcrumb" }
                                    Breadcrumb { items: breadcrumb_items.clone(), separator: ">".to_string() }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Navigation menu" }
                                    NavigationMenu { items: navigation_items.clone() }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Menubar" }
                                    Menubar {
                                        menus: menubar_menus.clone(),
                                        on_select: move |value| {
                                            let mut signal = menubar_selection_setter.clone();
                                            signal.set(format!("Menubar selected: {value}"));
                                        },
                                    }
                                    SpanHelper { "{menubar_selection()}" }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Pagination" }
                                    Pagination {
                                        total_pages: total_pages,
                                        current_page: pagination_current(),
                                        on_page_change: move |page| {
                                            let mut signal = pagination_setter.clone();
                                            signal.set(page);
                                        },
                                    }
                                    SpanHelper { "{pagination_summary()}" }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Steps" }
                                    Steps {
                                        steps: steps_items.clone(),
                                        current: steps_current(),
                                    }
                                    div { class: "ui-cluster",
                                        Button {
                                            variant: ButtonVariant::Outline,
                                            size: ButtonSize::Sm,
                                            on_click: move |_| {
                                                let mut signal = steps_setter.clone();
                                                let prev = signal().saturating_sub(1).max(1);
                                                signal.set(prev);
                                            },
                                            "Previous"
                                        }
                                        Button {
                                            size: ButtonSize::Sm,
                                            on_click: move |_| {
                                                let mut signal = steps_setter.clone();
                                                let next = (signal() + 1).min(steps_total);
                                                signal.set(next);
                                            },
                                            "Next"
                                        }
                                    }
                                    SpanHelper { "{steps_summary()}" }
                                }
                            }
                        }
                    }

                    div {
                        style: full_width_style,
                        Card {
                            CardHeader {
                                CardTitle { "Structural navigation" }
                                CardDescription { "Collapsible sidebar layout with grouped menus." }
                            }
                            CardContent {
                                SidebarLayout {
                                    Sidebar {
                                        collapsed: collapsed_state,
                                        SidebarHeader {
                                            div { class: "ui-sidebar-button-body",
                                                span { class: "ui-sidebar-icon", "⚡" }
                                                span { class: "ui-sidebar-text",
                                                    span { class: "ui-sidebar-label", "Acme HQ" }
                                                    span { class: "ui-sidebar-description", "Operations console" }
                                                }
                                            }
                                        }
                                        SidebarContent {
                                            SidebarGroup {
                                                SidebarGroupLabel { "Workspace" }
                                                SidebarGroupContent {
                                                    SidebarMenu {
                                                        SidebarMenuItem {
                                                        SidebarMenuButton {
                                                                label: "Analytics",
                                                                description: Some("Track KPIs and trends".into()),
                                                                icon: Some("📊".into()),
                                                                active: is_analytics_active,
                                                                on_click: move |_| {
                                                                    let mut signal = sidebar_active_setter.clone();
                                                                    signal.set("analytics".to_string());
                                                                },
                                                            }
                                                        }
                                                        SidebarMenuItem {
                                                        SidebarMenuButton {
                                                                label: "CRM",
                                                                description: Some("Manage customer pipeline".into()),
                                                                icon: Some("👥".into()),
                                                                active: is_crm_active,
                                                                on_click: move |_| {
                                                                    let mut signal = sidebar_active_setter.clone();
                                                                    signal.set("crm".to_string());
                                                                },
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            SidebarSeparator {}
                                            SidebarGroup {
                                                SidebarGroupLabel { "Reporting" }
                                                SidebarGroupContent {
                                                    SidebarMenu {
                                                        SidebarMenuItem {
                                                        SidebarMenuButton {
                                                                label: "Billing",
                                                                description: Some("Invoices, usage, balances".into()),
                                                                icon: Some("💳".into()),
                                                                badge: Some("8".into()),
                                                                active: is_billing_active,
                                                                on_click: move |_| {
                                                                    let mut signal = sidebar_active_setter.clone();
                                                                    signal.set("billing".to_string());
                                                                },
                                                            }
                                                        }
                                                        SidebarMenuItem {
                                                        SidebarMenuButton {
                                                                label: "Settings",
                                                                description: Some("Themes, tokens, notifications".into()),
                                                                icon: Some("⚙️".into()),
                                                                active: is_settings_active,
                                                                on_click: move |_| {
                                                                    let mut signal = sidebar_active_setter.clone();
                                                                    signal.set("settings".to_string());
                                                                },
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        SidebarFooter {
                                            SidebarTrigger {
                                                collapsed: collapsed_state,
                                                label: Some("Toggle sidebar".to_string()),
                                                on_toggle: move |next| {
                                                    let mut signal = sidebar_collapsed_setter.clone();
                                                    signal.set(next);
                                                },
                                            }
                                        }
                                    }
                                    SidebarInset {
                                        class: "ui-stack",
                                        h3 { style: "font-size: 1.2rem; font-weight: 600;", "{sidebar_title}" }
                                        p {
                                            style: "color: hsl(var(--muted-foreground)); max-width: 460px;",
                                            "{sidebar_body}"
                                        }
                                        SpanHelper { "Use the sidebar to swap the focused surface." }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Selection controls" }
                                CardDescription { "Checkboxes, switches, and radio groups stay in sync with signals." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    div { class: "ui-cluster",
                                        Checkbox {
                                            id: Some("newsletter-opt".to_string()),
                                            checked: newsletter_opt_in(),
                                            on_checked_change: move |state| newsletter_opt_in_setter.clone().set(state),
                                        }
                                        Label { html_for: "newsletter-opt", "Subscribe to newsletter" }
                                    }
                                    div { class: "ui-cluster",
                                        Label { html_for: "dark-mode", "Dark mode" }
                                        Switch {
                                            id: Some("dark-mode".to_string()),
                                            checked: dark_mode(),
                                            on_checked_change: move |state| dark_mode_setter.clone().set(state),
                                        }
                                    }
                                    Separator { style: "margin: 0.75rem 0;" }
                                    RadioGroup {
                                        default_value: contact_method(),
                                        on_value_change: move |value| contact_method_setter.clone().set(value),
                                        div { class: "ui-stack",
                                            div { class: "ui-cluster",
                                                RadioGroupItem { id: Some("contact-email".to_string()), value: "email" }
                                                Label { html_for: "contact-email", "Email" }
                                            }
                                            div { class: "ui-cluster",
                                                RadioGroupItem { id: Some("contact-sms".to_string()), value: "sms" }
                                                Label { html_for: "contact-sms", "SMS" }
                                            }
                                            div { class: "ui-cluster",
                                                RadioGroupItem { id: Some("contact-call".to_string()), value: "call" }
                                                Label { html_for: "contact-call", "Phone call" }
                                            }
                                        }
                                    }
                                    SpanHelper { "{contact_text()}" }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Command & context" }
                                CardDescription { "Command palette filtering and contextual menus." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    SpanHelper { "Command palette" }
                                    CommandPalette {
                                        items: command_items.clone(),
                                        on_select: move |value| {
                                            let mut signal = command_selection_setter.clone();
                                            signal.set(format!("Command selected: {value}"));
                                        },
                                    }
                                    SpanHelper { "{command_selection()}" }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Context menu" }
                                    ContextMenu {
                                        items: context_items.clone(),
                                        on_select: move |value| {
                                            let mut signal = context_selection_setter.clone();
                                            signal.set(format!("Context action: {value}"));
                                        },
                                        div {
                                            style: "padding: 1.5rem; border: 1px dashed hsl(var(--border)); border-radius: var(--radius); text-align: center;",
                                            "Right click anywhere in this box"
                                        }
                                    }
                                    SpanHelper { "{context_selection()}" }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Tabs & panels" }
                                CardDescription { "Tabbed navigation with content surfaces that stay in sync." }
                            }
                            CardContent {
                                Tabs {
                                    default_value: "overview",
                                    TabsList {
                                        TabsTrigger { value: "overview", "Overview" }
                                        TabsTrigger { value: "analytics", "Analytics" }
                                        TabsTrigger { value: "reports", "Reports" }
                                    }
                                    TabsContent {
                                        value: "overview",
                                        div { class: "ui-stack",
                                            Label { html_for: "overview-search", "Search" }
                                            Input { id: "overview-search", placeholder: "Search docs..." }
                                            SpanHelper { "Triggers share the same focus ring and sizing as the original UI kit." }
                                        }
                                    }
                                    TabsContent {
                                        value: "analytics",
                                        div { class: "ui-stack",
                                            SpanHelper { "Analytics aggregates live metrics and shows their progress." }
                                            Progress { value: 64.0, max: 100.0 }
                                        }
                                    }
                                    TabsContent {
                                        value: "reports",
                                        div { class: "ui-stack",
                                            SpanHelper { "Generate PDF, CSV, or scheduled exports directly from here." }
                                            Button { variant: ButtonVariant::Secondary, "Create report" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: full_width_style,
                        Card {
                            CardHeader {
                                CardTitle { "Dialogs & overlays" }
                                CardDescription { "Popover, hover card, dialogs, sheet, and toast examples." }
                            }
                            CardContent {
                                div { class: "ui-cluster",
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        on_click: move |_| {
                                            let mut signal = dialog_signal.clone();
                                            signal.set(true);
                                        },
                                        "Open dialog"
                                    }
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        on_click: move |_| {
                                            let mut signal = sheet_signal.clone();
                                            signal.set(true);
                                        },
                                        "Open sheet"
                                    }
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        on_click: move |_| {
                                            let mut signal = toast_signal.clone();
                                            signal.set(true);
                                        },
                                        "Notify me"
                                    }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Popover" }
                                    Popover {
                                        placement: "bottom".to_string(),
                                        trigger: rsx! { Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Toggle popover" } },
                                        content: rsx! { SpanHelper { "Choose the dialog or sheet you want to configure." } },
                                    }
                                }
                                div { class: "ui-stack",
                                    SpanHelper { "Hover card" }
                                    HoverCard {
                                        trigger: rsx! { Badge { variant: BadgeVariant::Secondary, "Hover me" } },
                                        content: rsx! { span { style: "font-size: 0.8rem; color: hsl(var(--muted-foreground));", "Preview contextual information instantly." } },
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: single_column_style,
                        Card {
                            CardHeader {
                                CardTitle { "Alerts & extras" }
                                CardDescription { "Feedback surfaces, accordions, and avatar fallbacks." }
                            }
                            CardContent {
                                div { class: "ui-stack",
                                    Alert {
                                        title: Some("Heads up!".to_string()),
                                        "We just shipped async server functions to production."
                                    }
                                    Alert {
                                        variant: AlertVariant::Destructive,
                                        title: Some("Deployment failed".to_string()),
                                        "Check the build logs and retry once the issue is resolved."
                                    }
                                }
                                Separator { style: "margin: 1rem 0;" }
                                Accordion {
                                    collapsible: true,
                                    default_value: Some("item-1".to_string()),
                                    AccordionItem {
                                        value: "item-1".to_string(),
                                        AccordionTrigger { "What is shadcn/ui?" }
                                        AccordionContent {
                                            "A collection of unstyled, accessible primitives built on top of Radix, ready for your design system."
                                        }
                                    }
                                    AccordionItem {
                                        value: "item-2".to_string(),
                                        AccordionTrigger { "Does this work with Dioxus?" }
                                        AccordionContent {
                                            "Yes! These components mirror the shadcn/ui ergonomics using Dioxus 0.7 signals."
                                        }
                                    }
                                }
                                Separator { style: "margin: 1rem 0;" }
                                div { class: "ui-cluster",
                                    Tooltip {
                                        label: "Ada Lovelace",
                                        Avatar {
                                            alt: Some("Ada Lovelace".to_string()),
                                            fallback: Some("AL".to_string()),
                                        }
                                    }
                                    Avatar {
                                        alt: Some("Grace Hopper".to_string()),
                                        fallback: Some("GH".to_string()),
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Dialog {
                open: dialog_signal.clone(),
                title: Some("Create project".to_string()),
                description: Some("Configure the new analytics workspace.".to_string()),
                div { class: "ui-stack",
                    Label { html_for: "dialog-name", "Project name" }
                    Input { id: "dialog-name", placeholder: "Analytics redesign" }
                }
            }
            Sheet {
                open: sheet_signal.clone(),
                side: SheetSide::Right,
                title: Some("Activity log".to_string()),
                description: Some("Review the latest changes from your teammates.".to_string()),
                div {
                    class: "ui-stack",
                    SpanHelper { "Today" }
                    ul {
                        style: "display: flex; flex-direction: column; gap: 0.5rem; font-size: 0.85rem;",
                        li { "Maria added new metrics to the dashboard." }
                        li { "Evan approved the Q2 launch plan." }
                        li { "Ada commented on revenue projections." }
                    }
                }
            }
            ToastViewport {
                Toast {
                    open: toast_open(),
                    title: Some("Changes saved".to_string()),
                    description: Some("We synced your workspace preferences.".to_string()),
                    on_close: move |_| {
                        let mut signal = toast_signal.clone();
                        signal.set(false);
                    },
                }
            }
        }
    }
}

#[component]
fn SpanHelper(children: Element) -> Element {
    rsx! { span { class: "ui-field-helper", {children} } }
}
