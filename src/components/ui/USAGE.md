# UI 组件使用手册

本手册针对 `src/components/ui` 下的 **Dioxus 0.7** 组件库，按分类说明每个组件（含子组件）与配套数据结构的用途、关键属性以及推荐写法。示例全部基于 `#[component]` 与 `Signal` 状态模型，确保可以直接粘贴到项目中演练。

- [基础展示组件](#基础展示组件)
- [布局与容器](#布局与容器)
- [表单与输入控件](#表单与输入控件)
- [导航与结构](#导航与结构)
- [数据展示与日期](#数据展示与日期)
- [反馈与浮层](#反馈与浮层)

---

## 基础展示组件

### Button / ButtonVariant / ButtonSize

主按钮组件，视觉样式与 shadcn 保持一致。

- **关键 props**：`variant: ButtonVariant`（Default/Secondary/Destructive/Outline/Ghost/Link/Icon）、`size: ButtonSize`（Default/Sm/Lg/Icon）、`on_click`.
- 使用 `disabled` 控制禁用，`class` 可追加自定义类。

```rust
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

#[component]
fn ButtonSamples() -> Element {
    rsx! {
        div { class: "ui-stack gap-2",
            Button { "保存" }
            Button { variant: ButtonVariant::Destructive, "删除" }
            Button { size: ButtonSize::Icon, "⚙" }
        }
    }
}
```

### Badge / BadgeVariant

用于展示状态或标签。

- `variant`（Default/Secondary/Outline/Destructive）映射到 `data-variant`。
- 直接包裹文本或图标即可。

```rust
use crate::components::ui::{Badge, BadgeVariant};

#[component]
fn BadgeSamples() -> Element {
    rsx! {
        div { class: "ui-cluster gap-2",
            Badge { "默认" }
            Badge { variant: BadgeVariant::Outline, "草稿" }
            Badge { variant: BadgeVariant::Destructive, "危险" }
        }
    }
}
```

### Avatar

头像组件，可在图片加载失败时展示缩写。

- `src` 为头像地址，`fallback` 显示自定义文本（默认取 `alt` 首字母）。
- 会监听 `img` 的 `onerror`/`onload` 自动切换。

```rust
use crate::components::ui::Avatar;

#[component]
fn AvatarSample() -> Element {
    rsx! {
        Avatar {
            src: Some("https://example.com/me.png".into()),
            alt: Some("Jane Doe".into()),
        }
    }
}
```

### Alert / AlertVariant

状态提示块。

- `variant`：Default 或 Destructive。
- 可选 `title`，正文通过 `children`。

```rust
use crate::components::ui::{Alert, AlertVariant};

#[component]
fn AlertSample() -> Element {
    rsx! {
        Alert {
            variant: AlertVariant::Destructive,
            title: Some("发布失败".into()),
            "请检查网络或稍后再试。"
        }
    }
}
```

### Progress

线性进度条。

- 传入 `value` 与 `max`，组件内部计算百分比。
- 可用 `class` 替换样式。

```rust
use crate::components::ui::Progress;

#[component]
fn ProgressSample() -> Element {
    rsx! { Progress { value: 32.0, max: 100.0 } }
}
```

### Skeleton

骨架屏占位。

- 可传 `width`/`height`/`radius`（CSS 长度或百分比）。

```rust
use crate::components::ui::Skeleton;

#[component]
fn SkeletonSample() -> Element {
    rsx! {
        Skeleton {
            width: Some("240px".into()),
            height: Some("48px".into()),
            radius: Some("12px".into()),
        }
    }
}
```

### Separator / SeparatorOrientation

分割线。

- `orientation` 控制横向或纵向（Horizontal/Vertical）。
- `style` 追加内联样式。

```rust
use crate::components::ui::{Separator, SeparatorOrientation};

#[component]
fn SeparatorSample() -> Element {
    rsx! {
        div { class: "ui-stack gap-2",
            "上方内容"
            Separator {}
            "下方内容"
            Separator {
                orientation: SeparatorOrientation::Vertical,
                style: Some("height: 40px;".into()),
            }
        }
    }
}
```

### AspectRatio

用于固定宽高比的媒体容器。`ratio` 默认 1:1，内部子元素会自动伸缩。

```rust
use crate::components::ui::AspectRatio;

#[component]
fn AspectRatioSample() -> Element {
    rsx! {
        AspectRatio { ratio: 16.0 / 9.0,
            img {
                src: "https://picsum.photos/800/450",
                alt: "示例图片",
                style: "width: 100%; height: 100%; object-fit: cover;",
            }
        }
    }
}
```

---

## 布局与容器

### Card 系列（Card / CardHeader / CardTitle / CardDescription / CardContent / CardFooter）

卡片容器，子组件负责分区。

```rust
use crate::components::ui::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Button};

#[component]
fn CardSample() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "团队概览" }
                CardDescription { "本月新增 24 位成员" }
            }
            CardContent {
                p { "更多内容放在这里…" }
            }
            CardFooter {
                Button { "查看详情" }
            }
        }
    }
}
```

### ScrollArea

限制高度并展示滚动条。

- 通过 `max_height` 或 `style` 设置滚动容器大小。

```rust
use crate::components::ui::ScrollArea;

#[component]
fn ScrollAreaSample() -> Element {
    rsx! {
        ScrollArea { max_height: Some("200px".into()),
            ul {
                for i in 1..=20 {
                    li { "列表项 #{i}" }
                }
            }
        }
    }
}
```

### ResizablePanels

可拖拽的双列布局。

- `initial`/`min`/`max` 控制百分比分布；`orientation` 可设 Horizontal / Vertical。
- `on_resize` 回调返回当前主面板占比。

```rust
use crate::components::ui::{ResizableOrientation, ResizablePanels};
use dioxus::prelude::*;

#[component]
fn ResizableSample() -> Element {
    rsx! {
        ResizablePanels {
            orientation: ResizableOrientation::Horizontal,
            first: rsx! { div { class: "ui-pane-muted", "左侧面板" } },
            second: rsx! { div { class: "ui-pane-surface", "右侧面板" } },
        }
    }
}
```

### Sidebar 体系

包含构建侧边导航所需的所有子组件。

- `Sidebar` 控制折叠态，`SidebarTrigger` 提供折叠按钮。
- `SidebarLayout` + `SidebarInset` 布局主体与内容。
- `SidebarMenu`/`SidebarMenuItem`/`SidebarMenuButton` 组合导航项，可传 `description`、`badge`、`icon`、`href`。
- 其他辅助块：`SidebarRail`、`SidebarHeader`、`SidebarContent`、`SidebarFooter`、`SidebarSeparator`、`SidebarGroup`（含 `SidebarGroupLabel`、`SidebarGroupContent`）、`SidebarMenuBadge`。

```rust
use crate::components::ui::*;
use dioxus::prelude::*;

#[component]
fn SidebarSample() -> Element {
    let mut collapsed = use_signal(|| false);

    rsx! {
        SidebarLayout {
            Sidebar {
                collapsed: collapsed(),
                SidebarHeader { "LOGO" }
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "管理" }
                        SidebarGroupContent {
                            SidebarMenu {
                                SidebarMenuItem {
                                    SidebarMenuButton {
                                        label: "仪表盘",
                                        icon: Some("🏠".into()),
                                        active: true,
                                    }
                                }
                                SidebarMenuItem {
                                    SidebarMenuButton {
                                        label: "订单",
                                        description: Some("最新交易记录".into()),
                                        badge: Some("8".into()),
                                    }
                                }
                            }
                        }
                    }
                }
                SidebarFooter {
                    SidebarSeparator {}
                    SidebarMenuBadge { text: "v1.2.0" }
                }
            }
            SidebarInset {
                header {
                    SidebarTrigger {
                        collapsed: collapsed(),
                        on_toggle: move |next| collapsed.set(!next),
                    }
                }
                main { class: "ui-surface p-6", "右侧内容区" }
            }
        }
    }
}
```

---

## 表单与输入控件

### Input

标准文本输入框。

- 支持受控与非受控：传 `value` 或 `default_value`。
- 常用属性：`placeholder`、`r#type`、`disabled`、`readonly`、`on_input`、`on_change`。

```rust
use crate::components::ui::Input;
use dioxus::prelude::*;

#[component]
fn InputSample() -> Element {
    let mut name = use_signal(|| "Alice".to_string());

    rsx! {
        Input {
            value: Some(name()),
            placeholder: Some("请输入姓名".into()),
            on_input: move |event| name.set(event.value()),
        }
    }
}
```

### Label

配合表单控件使用的 `<label>`。`html_for` 应与控件 `id` 匹配，`disabled` 会同步到 `data-disabled`。

```rust
use crate::components::ui::{Input, Label};

#[component]
fn LabelSample() -> Element {
    rsx! {
        div { class: "ui-stack gap-2",
            Label { html_for: Some("city".into()), "城市" }
            Input { id: Some("city".into()), placeholder: Some("请输入所在城市".into()) }
        }
    }
}
```

### Textarea

多行文本输入，API 与 `Input` 接近，额外提供 `rows`。

```rust
use crate::components::ui::Textarea;

#[component]
fn TextareaSample() -> Element {
    rsx! {
        Textarea {
            placeholder: Some("请输入备注".into()),
            rows: Some(6),
        }
    }
}
```

### Checkbox

- `checked`、`disabled`、`required` 控制状态。
- 可监听 `on_checked_change`（返回 bool）与 `on_input` / `on_change`。

```rust
use crate::components::ui::Checkbox;
use dioxus::prelude::*;

#[component]
fn CheckboxSample() -> Element {
    let mut agreed = use_signal(|| false);

    rsx! {
        Checkbox {
            checked: agreed(),
            on_checked_change: move |next| agreed.set(next),
            id: Some("terms".into()),
        }
    }
}
```

### RadioGroup / RadioGroupItem

互斥单选框集合。

- `default_value` 指定初始值；未传 `name` 时自动生成。
- `on_value_change` 返回选中的 `String`。

```rust
use crate::components::ui::{RadioGroup, RadioGroupItem};
use dioxus::prelude::*;

#[component]
fn RadioSample() -> Element {
    rsx! {
        RadioGroup {
            default_value: Some("email".into()),
            on_value_change: move |value| log::info!("选中 {value}"),
            RadioGroupItem { value: "email".into(), id: Some("email".into()) }
            label { r#for: "email", "邮件通知" }
            RadioGroupItem { value: "sms".into(), id: Some("sms".into()) }
            label { r#for: "sms", "短信通知" }
        }
    }
}
```

### Switch

语义化开关，内部同样使用 `<input type="checkbox">`。

```rust
use crate::components::ui::Switch;
use dioxus::prelude::*;

#[component]
fn SwitchSample() -> Element {
    let mut enabled = use_signal(|| true);

    rsx! {
        Switch {
            checked: enabled(),
            on_checked_change: move |next| enabled.set(next),
        }
    }
}
```

### Toggle

小型按钮式开关，一般与图标搭配。

```rust
use crate::components::ui::Toggle;

#[component]
fn ToggleSample() -> Element {
    rsx! {
        Toggle { pressed: true, "粗体" }
    }
}
```

### ToggleGroup / ToggleGroupItem / ToggleGroupMode

组合多个 Toggle。

- 将 `Signal<Vec<String>>` 传给 `ToggleGroup`，`mode` 支持 Single / Multiple。
- `on_value_change` 返回当前激活值列表。

```rust
use crate::components::ui::{ToggleGroup, ToggleGroupItem, ToggleGroupMode};
use dioxus::prelude::*;

#[component]
fn ToggleGroupSample() -> Element {
    let mut active = use_signal(|| vec!["bold".to_string()]);

    rsx! {
        ToggleGroup {
            values: active.clone(),
            mode: ToggleGroupMode::Multiple,
            on_value_change: move |values| active.set(values),
            ToggleGroupItem { value: "bold".into(), "B" }
            ToggleGroupItem { value: "italic".into(), "I" }
            ToggleGroupItem { value: "underline".into(), "U" }
        }
    }
}
```

### Slider

输入连续或离散数值。

- `min`/`max`/`step` 控制范围；`on_value_change` 直接返回 `f32`。

```rust
use crate::components::ui::Slider;
use dioxus::prelude::*;

#[component]
fn SliderSample() -> Element {
    let mut value = use_signal(|| 42.0f32);

    rsx! {
        Slider {
            value: value(),
            min: 0.0,
            max: 100.0,
            on_value_change: move |next| value.set(next),
        }
    }
}
```

### Select / SelectOption

下拉选择器，支持受控选中。

```rust
use crate::components::ui::{Select, SelectOption};
use dioxus::prelude::*;

#[component]
fn SelectSample() -> Element {
    let options = vec![
        SelectOption::new("待处理", "pending"),
        SelectOption::new("已发布", "published"),
    ];
    let mut selected = use_signal(|| Some("pending".to_string()));

    rsx! {
        Select {
            placeholder: "选择状态".into(),
            options: options.clone(),
            selected: selected(),
            on_change: move |value| selected.set(Some(value)),
        }
    }
}
```

### Combobox / ComboboxOption

带搜索框的选择器。

- `placeholder` 为按钮文案，`search_placeholder` 控制输入框占位。
- `options` 使用 `ComboboxOption::new` / `with_description` 构建。

```rust
use crate::components::ui::{Combobox, ComboboxOption};
use dioxus::prelude::*;

#[component]
fn ComboboxSample() -> Element {
    let options = vec![
        ComboboxOption::new("Bengal", "bengal").with_description("聪明好动"),
        ComboboxOption::new("Ragdoll", "ragdoll").with_description("温柔黏人"),
    ];
    let mut selected = use_signal(|| None::<String>);

    rsx! {
        Combobox {
            placeholder: "选择猫咪品种".into(),
            options: options.clone(),
            selected: selected(),
            on_select: move |value| selected.set(Some(value)),
        }
    }
}
```

### CommandPalette / CommandItem

类似 `⌘K` 的命令面板。

- `CommandItem::new` 创建项目，可追加 `.shortcut()` / `.group()`.
- `on_select` 返回命令值。

```rust
use crate::components::ui::{CommandItem, CommandPalette};

#[component]
fn CommandSample() -> Element {
    let items = vec![
        CommandItem::new("新建项目", "new_project").shortcut("⌘N"),
        CommandItem::new("打开设置", "settings").group("偏好"),
    ];

    rsx! {
        CommandPalette {
            items: items,
            on_select: move |value| log::info!("执行命令 {value}"),
        }
    }
}
```

### FileDropZone / FileMetadata

拖拽上传区域。

- `multiple` 控制多选，`accept` 填写 MIME 过滤。
- `on_files` 返回文件元数据列表。

```rust
use crate::components::ui::FileDropZone;
use dioxus::prelude::*;

#[component]
fn FileDropSample() -> Element {
    rsx! {
        FileDropZone {
            multiple: true,
            accept: Some("image/png,image/jpeg".into()),
            on_files: move |files| log::info!("共 {files:?}"),
        }
    }
}
```

### FormField / FormMessage / FormMessageVariant

组合标签、输入框与辅助文字。

- `error` 接受 `Signal<Option<String>>`，动态展示错误信息。
- `FormMessage` 可单独使用，`variant` 为 Helper/Error。

```rust
use crate::components::ui::{FormField, FormMessage, FormMessageVariant, Input};
use dioxus::prelude::*;

#[component]
fn FormFieldSample() -> Element {
    let mut error = use_signal(|| None::<String>);

    rsx! {
        FormField {
            label: Some("邮箱".into()),
            helper_text: Some("我们不会泄露您的邮箱".into()),
            error: Some(error.clone()),
            id: Some("email".into()),
            Input {
                id: Some("email".into()),
                on_input: move |event| {
                    let value = event.value();
                    error.set(if value.contains('@') { None } else { Some("邮箱格式不正确".into()) });
                },
            }
        }
        FormMessage {
            variant: FormMessageVariant::Helper,
            class: Some("mt-2 text-xs text-muted-foreground".into()),
            "额外说明文本"
        }
    }
}
```

---

## 导航与结构

### Breadcrumb / Crumb

层级导航。

- 使用 `Crumb::new(label, href)` 构建列表；`separator` 默认 "/"。

```rust
use crate::components::ui::{Breadcrumb, Crumb};

#[component]
fn BreadcrumbSample() -> Element {
    let items = vec![
        Crumb::new("首页", Some("/")),
        Crumb::new("设置", Some("/settings")),
        Crumb::new("通知", None::<String>),
    ];

    rsx! { Breadcrumb { items: items, separator: "›".to_string() } }
}
```

### Pagination

页码导航。

- 传 `total_pages` 与 `current_page`，`on_page_change` 返回目标页。

```rust
use crate::components::ui::Pagination;

#[component]
fn PaginationSample() -> Element {
    rsx! {
        Pagination {
            total_pages: 12,
            current_page: 1,
            on_page_change: move |page| log::info!("跳转到第 {page} 页"),
        }
    }
}
```

### Tabs 系列（Tabs / TabsList / TabsTrigger / TabsContent / TabsOrientation）

选项卡容器，内部自动管理 `Signal`。

```rust
use crate::components::ui::{Tabs, TabsContent, TabsList, TabsTrigger, TabsOrientation};

#[component]
fn TabsSample() -> Element {
    rsx! {
        Tabs {
            default_value: "account".into(),
            orientation: TabsOrientation::Horizontal,
            on_value_change: move |value| log::info!("切换到 {value}"),
            TabsList {
                TabsTrigger { value: "account".into(), "账户" }
                TabsTrigger { value: "password".into(), "密码" }
            }
            TabsContent { value: "account".into(), "账户配置表单…" }
            TabsContent { value: "password".into(), "修改密码表单…" }
        }
    }
}
```

### Steps / StepItem

展示流程或进度。

- `steps` 为 `StepItem` 列表，`current` 表示当前步骤（1-based）。

```rust
use crate::components::ui::{StepItem, Steps};

#[component]
fn StepsSample() -> Element {
    let steps = vec![
        StepItem::new("创建账户", None::<String>),
        StepItem::new("填写资料", Some("约 2 分钟")),
        StepItem::new("完成", None::<String>),
    ];

    rsx! { Steps { steps: steps, current: 2 } }
}
```

### Accordion 系列（Accordion / AccordionItem / AccordionTrigger / AccordionContent）

折叠面板，内部通过上下文维持当前展开项。

- `collapsible` 允许再次点击收起，`default_value` 指定默认展开的 `value`。
- `AccordionItem` 的 `value` 必须在同一 `Accordion` 中唯一。

```rust
use crate::components::ui::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
fn AccordionSample() -> Element {
    rsx! {
        Accordion {
            collapsible: true,
            default_value: Some("billing".into()),
            AccordionItem { value: "profile".into(),
                AccordionTrigger { "个人信息" }
                AccordionContent { "更新姓名与头像" }
            }
            AccordionItem { value: "billing".into(),
                AccordionTrigger { "账单信息" }
                AccordionContent { "绑定公司信用卡与发票抬头" }
            }
        }
    }
}
```

### Collapsible 系列（Collapsible / CollapsibleTrigger / CollapsibleContent）

与 Accordion 类似，但由外部 `Signal<bool>` 控制。

```rust
use crate::components::ui::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use dioxus::prelude::*;

#[component]
fn CollapsibleSample() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Collapsible {
            open: open.clone(),
            on_open_change: move |next| open.set(next),
            CollapsibleTrigger { "展开更多设置" }
            CollapsibleContent { "隐藏的专业配置…" }
        }
    }
}
```

### NavigationMenu / NavigationItem

悬浮式导航面板。

```rust
use crate::components::ui::{NavigationItem, NavigationMenu};

#[component]
fn NavigationMenuSample() -> Element {
    let items = vec![
        NavigationItem::new("产品", "/products", Some("探索我们提供的模块")),
        NavigationItem::new("价格", "/pricing", Some("对比不同套餐")),
    ];

    rsx! { NavigationMenu { items: items } }
}
```

### Menubar / MenubarMenu / MenubarItem

桌面应用风格的顶部菜单。

- `MenubarMenu::new(label, items)` 组合多个菜单。
- `MenubarItem::new` 可附带 `.shortcut()` 或 `.destructive()`.

```rust
use crate::components::ui::{Menubar, MenubarItem, MenubarMenu};

#[component]
fn MenubarSample() -> Element {
    let menus = vec![
        MenubarMenu::new("文件", vec![
            MenubarItem::new("新建", "new").shortcut("⌘N"),
            MenubarItem::new("退出", "quit").destructive(),
        ]),
        MenubarMenu::new("帮助", vec![
            MenubarItem::new("文档", "docs"),
        ]),
    ];

    rsx! {
        Menubar {
            menus: menus,
            on_select: move |value| log::info!("菜单 {value} 被点击"),
        }
    }
}
```

### DropdownMenu / DropdownMenuItem / DropdownItemVariant

按钮触发的短列表菜单。

```rust
use crate::components::ui::{DropdownMenu, DropdownMenuItem};

#[component]
fn DropdownSample() -> Element {
    let items = vec![
        DropdownMenuItem::new("查看详情", "view"),
        DropdownMenuItem::new("删除", "delete").destructive(),
    ];

    rsx! {
        DropdownMenu {
            label: "操作".into(),
            items: items,
            on_select: move |value| log::info!("选择 {value}"),
        }
    }
}
```

### SidebarTrigger（已在 “Sidebar 体系” 示例中使用）

用于折叠/展开主侧栏，`on_toggle` 回调提供目标状态（true 表示展开）。

---

## 数据展示与日期

### Table 系列（Table / TableHeader / TableBody / TableFooter / TableRow / TableHead / TableCell / TableCaption）

构建基础表格结构。

```rust
use crate::components::ui::*;

#[component]
fn TableSample() -> Element {
    rsx! {
        Table {
            TableCaption { "近七天销售额" }
            TableHeader {
                TableRow {
                    TableHead { "日期" }
                    TableHead { "订单数" }
                    TableHead { "金额" }
                }
            }
            TableBody {
                for (date, orders, amount) in [("周一", 32, "¥3,200"), ("周二", 28, "¥2,880")] {
                    TableRow {
                        TableCell { "{date}" }
                        TableCell { "{orders}" }
                        TableCell { "{amount}" }
                    }
                }
            }
        }
    }
}
```

### Calendar

单月日期选择器。

- `initial_month: NaiveDate` 指定初始月份；`selected` 为当前日期。
- `on_select` 返回选中的 `NaiveDate`。

```rust
use crate::components::ui::Calendar;
use chrono::NaiveDate;
use dioxus::prelude::*;

#[component]
fn CalendarSample() -> Element {
    let mut selected = use_signal(|| None::<NaiveDate>);

    rsx! {
        Calendar {
            initial_month: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            selected: selected(),
            on_select: move |date| selected.set(Some(date)),
        }
    }
}
```

### DateRangePicker / DateRange

选择日期范围。

- 接受 `Signal<Option<DateRange>>`；`DateRange::new(start, end)` 自动排序。
- `on_change` 返回新的区间或 `None`。

```rust
use crate::components::ui::{DateRange, DateRangePicker};
use chrono::NaiveDate;
use dioxus::prelude::*;

#[component]
fn DateRangeSample() -> Element {
    let mut range = use_signal(|| None::<DateRange>);

    rsx! {
        DateRangePicker {
            value: range.clone(),
            on_change: move |value| range.set(value),
            initial_month: Some(NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()),
        }
    }
}
```

### Pagination（见“导航与结构”）

### Avatar / Badge / Progress / Skeleton（见“基础展示组件”）

---

## 反馈与浮层

### ToastViewport / Toast

轻量通知。

- `ToastViewport` 为容器，可在其中渲染多个 `Toast`。
- `Toast` 通过 `open` 控制显示，`on_close` 在关闭时回调。

```rust
use crate::components::ui::{Toast, ToastViewport};

#[component]
fn ToastSample() -> Element {
    rsx! {
        ToastViewport {
            Toast {
                open: true,
                title: Some("已保存".into()),
                description: Some("更改已同步到云端".into()),
            }
        }
    }
}
```

### Dialog

模态对话框，由 `Signal<bool>` 控制开关。

- 点击遮罩或底部按钮会自动关闭，并调用 `on_close`。

```rust
use crate::components::ui::Dialog;
use dioxus::prelude::*;

#[component]
fn DialogSample() -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        Dialog {
            open: open.clone(),
            title: Some("重启服务".into()),
            description: Some("重启期间 API 会短暂不可用。".into()),
            on_close: move |_| open.set(false),
            div { class: "ui-stack gap-2",
                "确认要立即重启吗？"
            }
        }
    }
}
```

### Sheet / SheetSide

侧滑面板，与 Dialog 类似，通过 `Signal<bool>` 控制。

- `side` 选择从左或右滑入。

```rust
use crate::components::ui::{Sheet, SheetSide};
use dioxus::prelude::*;

#[component]
fn SheetSample() -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        Sheet {
            open: open.clone(),
            side: SheetSide::Right,
            title: Some("筛选条件".into()),
            description: Some("调整后自动刷新列表".into()),
            on_close: move |_| open.set(false),
            div { class: "ui-stack gap-3", "过滤条件内容…" }
        }
    }
}
```

### HoverCard

悬停展示额外信息，传入 `trigger` 与 `content` 两个 `Element`。

```rust
use crate::components::ui::HoverCard;

#[component]
fn HoverCardSample() -> Element {
    rsx! {
        HoverCard {
            trigger: rsx! { span { "查看作者" } },
            content: rsx! {
                div { class: "ui-stack gap-1 p-2",
                    strong { "Jane Doe" }
                    span { "产品经理 · 上海" }
                }
            },
        }
    }
}
```

### Popover

点击触发的气泡层。

- `placement` 为 `"top" | "bottom" | "left" | "right"`。

```rust
use crate::components::ui::Popover;

#[component]
fn PopoverSample() -> Element {
    rsx! {
        Popover {
            placement: "bottom".into(),
            trigger: rsx! { Button { "打开弹层" } },
            content: rsx! { div { class: "p-3 text-sm", "自定义内容、自定义表单…" } },
        }
    }
}
```

### Tooltip

简单提示，`label` 为文字，`delay_ms` 预留给自定义延迟（当前实现即时显示）。

```rust
use crate::components::ui::Tooltip;

#[component]
fn TooltipSample() -> Element {
    rsx! {
        Tooltip {
            label: "复制链接".into(),
            span { "🔗" }
        }
    }
}
```

### ContextMenu / ContextItem

右键菜单。

- `items` 使用 `ContextItem::new`，可链式 `.destructive()`。
- `on_select` 返回选中的 `value`。

```rust
use crate::components::ui::{ContextItem, ContextMenu};

#[component]
fn ContextMenuSample() -> Element {
    let items = vec![
        ContextItem::new("复制", "copy"),
        ContextItem::new("删除", "delete").destructive(),
    ];

    rsx! {
        ContextMenu {
            items: items,
            on_select: move |value| log::info!("右键操作：{value}"),
            div { class: "p-6 border rounded-md", "右键我试试看" }
        }
    }
}
```

### DropdownMenu / Menubar / NavigationMenu（见“导航与结构”）

### Toast / Alert / Dialog / Sheet（已分别示例）

---

以上示例覆盖 `ui` 目录内所有组件与配套数据结构。复制示例到任意 Dioxus 组件中即可快速验证样式与交互，并可按需调整 props 或类名来匹配业务需求。若需要更多范例，可参考 `src/views/home.rs` 的组件展示。***
