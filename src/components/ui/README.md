# UI 组件开发指引

这里的 `ui` 模块封装了一套基于 **Dioxus 0.7** 的 shadcn 风格原子组件。目标是保持 API 与 React 版 shadcn 接近，同时遵循 Dioxus 0.7 的信号（`use_signal`/`Signal<T>`）模型。本文档记录扩展组件时需要注意的约定。

## 文件组织

- 每个组件一个独立的 `*.rs` 文件，统一在 `mod.rs` 中 `mod xxx;` 并 `pub use xxx::*;` 暴露。
- 公共类型（例如 `enum`、`struct` props 容器）与组件实现写在同一文件；避免跨文件循环依赖。
- 若组件需要多子组件（例如 `CardHeader`/`CardFooter`），与主组件放在同文件中维护。

## CSS 与主题

- 所有样式都集中在 `assets/styling/shadcn.css`。
  - 请尽量使用现有的工具类（例如 `ui-stack`、`ui-cluster`）。
  - 新增组件样式时，使用前缀 `ui-组件名`，并保持 shadcn 命名习惯。
- 暗色模式依靠 `:root` 与 `[data-theme="dark"]` 的 CSS 变量切换，组件代码不要直接写死颜色。

## API 约定

- Props 使用 Dioxus 0.7 推荐的 `#[component]` + 函数参数方式。
  - 文本、标识符等参数提供 `#[props(into)]`，允许直接传入 `&'static str`。
  - 布尔开关提供 `#[props(default)]`，保证默认行为与 shadcn 原版一致。
  - 事件回调使用 `Option<EventHandler<_>>`，命名参考 shadcn，如 `on_change`、`on_select`。
- 如果组件需要内部状态（如弹层开关），优先让外部驱动：通过 `Signal<bool>` 或回调交还给父组件。仅在确有必要时内部使用 `use_signal`。
- 需要派生的扩展类型（例如 `BadgeVariant`）实现 `Default` 并提供 `as_str()`，这样方便映射到 `data-variant` 等属性。

## 信号与回调

- 事件闭包里修改 signal 时注意 `let mut signal = signal.clone();`，避免借用错误。
- 尽可能让组件保持**纯函数式**：根据 props 渲染，不在组件本地缓存额外状态。
- 对外暴露的 `Signal<T>` props 均使用 **owned 类型**，例如 `Signal<bool>`，避免引用生命周期复杂化。

## 示例与文档

- 每次新增组件，请在 `src/views/home.rs` 的 `UiShowcase` 中添加演示，方便快速验证样式。
- 如需说明组件使用方式，可在此 README 末尾追加小节或添加注释。

## 命名与公共导出

- 组件名保持首字母大写，必要时使用驼峰。例如 `NavigationMenu`、`CommandPalette`。
- 如果组件包含多个子组件，导出所有子组件，引用方式保持 `ui::CardHeader` 这种命名空间。

## 测试与检查

1. 修改后运行：

   ```bash
   cargo fmt
   cargo check
   ```

2. 通过 `dx serve --platform web` 手动验证交互（尤其是动画/暗色模式）。

## 常见组件类型参考

- **基础交互**：Button、Badge、Input、Select。
- **导航**：Breadcrumb、Pagination、Steps、Tabs、NavigationMenu、Menubar。
- **反馈**：Alert、Toast、Dialog、Sheet、Popover、HoverCard。
- **表单扩展**：CommandPalette、ContextMenu 等。

若要继续扩展（如 Calendar、Table、Dialog 组合组件等），请对照 shadcn 原版 API 与视觉，维持一致的 props 设计和 CSS 前缀，并补全示例。欢迎根据需要继续迭代！
