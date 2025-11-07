use crate::components::ui::{
    Avatar, Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardContent,
    CardDescription, CardFooter, CardHeader, CardTitle, CheckboxChipGroup, CheckboxChipOption,
    DateRange, DateRangePicker, Input, Label, Pagination, Popover, Select, SelectOption, Slider,
    Switch, Table, TableBody, TableCell, TableHead, TableHeader, TableRow, ToggleGroup,
    ToggleGroupItem, ToggleGroupMode,
};
use chrono::NaiveDate;
use dioxus::prelude::*;

const PAGE_SIZE: usize = 8;
const AVAILABLE_TAGS: &[&str] = &["加急", "赠品", "VIP", "缺货", "重复下单", "需回访"];

#[derive(Clone)]
struct Order {
    number: String,
    placed_on: NaiveDate,
    customer_name: String,
    customer_email: String,
    status: OrderStatus,
    payment_status: PaymentStatus,
    fulfillment_status: FulfillmentStatus,
    payment_method: PaymentMethod,
    channel: SalesChannel,
    total: f32,
    tags: Vec<String>,
    flagged: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OrderStatus {
    Draft,
    PendingPayment,
    Processing,
    Fulfilled,
    Cancelled,
}

impl OrderStatus {
    fn label(&self) -> &'static str {
        match self {
            OrderStatus::Draft => "草稿",
            OrderStatus::PendingPayment => "待支付",
            OrderStatus::Processing => "处理中",
            OrderStatus::Fulfilled => "已完成",
            OrderStatus::Cancelled => "已取消",
        }
    }

    fn key(&self) -> &'static str {
        match self {
            OrderStatus::Draft => "draft",
            OrderStatus::PendingPayment => "pending",
            OrderStatus::Processing => "processing",
            OrderStatus::Fulfilled => "fulfilled",
            OrderStatus::Cancelled => "cancelled",
        }
    }

    fn badge(&self) -> BadgeVariant {
        match self {
            OrderStatus::Draft => BadgeVariant::Secondary,
            OrderStatus::PendingPayment => BadgeVariant::Outline,
            OrderStatus::Processing => BadgeVariant::Default,
            OrderStatus::Fulfilled => BadgeVariant::Default,
            OrderStatus::Cancelled => BadgeVariant::Destructive,
        }
    }

    fn all() -> &'static [OrderStatus] {
        &[
            OrderStatus::Draft,
            OrderStatus::PendingPayment,
            OrderStatus::Processing,
            OrderStatus::Fulfilled,
            OrderStatus::Cancelled,
        ]
    }

    fn from_key(value: &str) -> Option<Self> {
        match value {
            "draft" => Some(OrderStatus::Draft),
            "pending" => Some(OrderStatus::PendingPayment),
            "processing" => Some(OrderStatus::Processing),
            "fulfilled" => Some(OrderStatus::Fulfilled),
            "cancelled" => Some(OrderStatus::Cancelled),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PaymentStatus {
    Pending,
    Paid,
    Refunded,
    Overdue,
}

impl PaymentStatus {
    fn label(&self) -> &'static str {
        match self {
            PaymentStatus::Pending => "待入账",
            PaymentStatus::Paid => "已支付",
            PaymentStatus::Refunded => "已退款",
            PaymentStatus::Overdue => "逾期",
        }
    }

    fn key(&self) -> &'static str {
        match self {
            PaymentStatus::Pending => "pending",
            PaymentStatus::Paid => "paid",
            PaymentStatus::Refunded => "refunded",
            PaymentStatus::Overdue => "overdue",
        }
    }

    fn all() -> &'static [PaymentStatus] {
        &[
            PaymentStatus::Pending,
            PaymentStatus::Paid,
            PaymentStatus::Refunded,
            PaymentStatus::Overdue,
        ]
    }

    fn from_key(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(PaymentStatus::Pending),
            "paid" => Some(PaymentStatus::Paid),
            "refunded" => Some(PaymentStatus::Refunded),
            "overdue" => Some(PaymentStatus::Overdue),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FulfillmentStatus {
    Unfulfilled,
    Picking,
    Shipped,
    Delivered,
    Returned,
}

impl FulfillmentStatus {
    fn label(&self) -> &'static str {
        match self {
            FulfillmentStatus::Unfulfilled => "待打包",
            FulfillmentStatus::Picking => "拣货中",
            FulfillmentStatus::Shipped => "运输中",
            FulfillmentStatus::Delivered => "已签收",
            FulfillmentStatus::Returned => "已退回",
        }
    }

    fn key(&self) -> &'static str {
        match self {
            FulfillmentStatus::Unfulfilled => "unfulfilled",
            FulfillmentStatus::Picking => "picking",
            FulfillmentStatus::Shipped => "shipped",
            FulfillmentStatus::Delivered => "delivered",
            FulfillmentStatus::Returned => "returned",
        }
    }

    fn all() -> &'static [FulfillmentStatus] {
        &[
            FulfillmentStatus::Unfulfilled,
            FulfillmentStatus::Picking,
            FulfillmentStatus::Shipped,
            FulfillmentStatus::Delivered,
            FulfillmentStatus::Returned,
        ]
    }

    fn from_key(value: &str) -> Option<Self> {
        match value {
            "unfulfilled" => Some(FulfillmentStatus::Unfulfilled),
            "picking" => Some(FulfillmentStatus::Picking),
            "shipped" => Some(FulfillmentStatus::Shipped),
            "delivered" => Some(FulfillmentStatus::Delivered),
            "returned" => Some(FulfillmentStatus::Returned),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SalesChannel {
    OnlineStore,
    Marketplace,
    Wholesale,
    PopUp,
    Subscription,
}

impl SalesChannel {
    fn label(&self) -> &'static str {
        match self {
            SalesChannel::OnlineStore => "官网商城",
            SalesChannel::Marketplace => "第三方平台",
            SalesChannel::Wholesale => "批发",
            SalesChannel::PopUp => "快闪店",
            SalesChannel::Subscription => "订阅",
        }
    }

    fn key(&self) -> &'static str {
        match self {
            SalesChannel::OnlineStore => "store",
            SalesChannel::Marketplace => "marketplace",
            SalesChannel::Wholesale => "wholesale",
            SalesChannel::PopUp => "popup",
            SalesChannel::Subscription => "subscription",
        }
    }

    fn all() -> &'static [SalesChannel] {
        &[
            SalesChannel::OnlineStore,
            SalesChannel::Marketplace,
            SalesChannel::Wholesale,
            SalesChannel::PopUp,
            SalesChannel::Subscription,
        ]
    }

    fn from_key(value: &str) -> Option<Self> {
        match value {
            "store" => Some(SalesChannel::OnlineStore),
            "marketplace" => Some(SalesChannel::Marketplace),
            "wholesale" => Some(SalesChannel::Wholesale),
            "popup" => Some(SalesChannel::PopUp),
            "subscription" => Some(SalesChannel::Subscription),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PaymentMethod {
    CreditCard,
    BankTransfer,
    Cash,
    Paypal,
    WechatPay,
    Alipay,
}

impl PaymentMethod {
    fn label(&self) -> &'static str {
        match self {
            PaymentMethod::CreditCard => "信用卡",
            PaymentMethod::BankTransfer => "银行转账",
            PaymentMethod::Cash => "现金",
            PaymentMethod::Paypal => "PayPal",
            PaymentMethod::WechatPay => "微信支付",
            PaymentMethod::Alipay => "支付宝",
        }
    }

    fn key(&self) -> &'static str {
        match self {
            PaymentMethod::CreditCard => "card",
            PaymentMethod::BankTransfer => "transfer",
            PaymentMethod::Cash => "cash",
            PaymentMethod::Paypal => "paypal",
            PaymentMethod::WechatPay => "wechat",
            PaymentMethod::Alipay => "alipay",
        }
    }

    fn all() -> &'static [PaymentMethod] {
        &[
            PaymentMethod::CreditCard,
            PaymentMethod::BankTransfer,
            PaymentMethod::Cash,
            PaymentMethod::Paypal,
            PaymentMethod::WechatPay,
            PaymentMethod::Alipay,
        ]
    }

    fn from_key(value: &str) -> Option<Self> {
        match value {
            "card" => Some(PaymentMethod::CreditCard),
            "transfer" => Some(PaymentMethod::BankTransfer),
            "cash" => Some(PaymentMethod::Cash),
            "paypal" => Some(PaymentMethod::Paypal),
            "wechat" => Some(PaymentMethod::WechatPay),
            "alipay" => Some(PaymentMethod::Alipay),
            _ => None,
        }
    }
}

fn seeded_orders() -> Vec<Order> {
    vec![
        Order {
            number: "DX-1050".to_string(),
            placed_on: date(2024, 7, 23),
            customer_name: "孙若水".to_string(),
            customer_email: "ruoshui@example.com".to_string(),
            status: OrderStatus::Processing,
            payment_status: PaymentStatus::Paid,
            fulfillment_status: FulfillmentStatus::Picking,
            payment_method: PaymentMethod::CreditCard,
            channel: SalesChannel::OnlineStore,
            total: 1288.0,
            tags: tags(&["VIP", "加急"]),
            flagged: true,
        },
        Order {
            number: "DX-1049".to_string(),
            placed_on: date(2024, 7, 22),
            customer_name: "李倩".to_string(),
            customer_email: "lian@example.com".to_string(),
            status: OrderStatus::PendingPayment,
            payment_status: PaymentStatus::Pending,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            payment_method: PaymentMethod::WechatPay,
            channel: SalesChannel::Marketplace,
            total: 342.0,
            tags: tags(&["需回访"]),
            flagged: true,
        },
        Order {
            number: "DX-1048".to_string(),
            placed_on: date(2024, 7, 21),
            customer_name: "Zoe Chen".to_string(),
            customer_email: "zoe@example.com".to_string(),
            status: OrderStatus::Processing,
            payment_status: PaymentStatus::Paid,
            fulfillment_status: FulfillmentStatus::Shipped,
            payment_method: PaymentMethod::Paypal,
            channel: SalesChannel::Marketplace,
            total: 812.5,
            tags: tags(&["加急"]),
            flagged: false,
        },
        Order {
            number: "DX-1047".to_string(),
            placed_on: date(2024, 7, 20),
            customer_name: "王宏".to_string(),
            customer_email: "hong@example.com".to_string(),
            status: OrderStatus::Fulfilled,
            payment_status: PaymentStatus::Paid,
            fulfillment_status: FulfillmentStatus::Delivered,
            payment_method: PaymentMethod::CreditCard,
            channel: SalesChannel::OnlineStore,
            total: 1560.0,
            tags: tags(&["VIP", "赠品"]),
            flagged: false,
        },
        Order {
            number: "DX-1046".to_string(),
            placed_on: date(2024, 7, 18),
            customer_name: "刘洋".to_string(),
            customer_email: "yang@example.com".to_string(),
            status: OrderStatus::Processing,
            payment_status: PaymentStatus::Overdue,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            payment_method: PaymentMethod::BankTransfer,
            channel: SalesChannel::Wholesale,
            total: 2890.4,
            tags: tags(&["缺货", "需回访"]),
            flagged: true,
        },
        Order {
            number: "DX-1045".to_string(),
            placed_on: date(2024, 7, 17),
            customer_name: "陈浩".to_string(),
            customer_email: "hao@example.com".to_string(),
            status: OrderStatus::Cancelled,
            payment_status: PaymentStatus::Refunded,
            fulfillment_status: FulfillmentStatus::Returned,
            payment_method: PaymentMethod::CreditCard,
            channel: SalesChannel::OnlineStore,
            total: 420.0,
            tags: tags(&["重复下单"]),
            flagged: false,
        },
        Order {
            number: "DX-1044".to_string(),
            placed_on: date(2024, 7, 16),
            customer_name: "Marvin Zhou".to_string(),
            customer_email: "marvin@example.com".to_string(),
            status: OrderStatus::Processing,
            payment_status: PaymentStatus::Paid,
            fulfillment_status: FulfillmentStatus::Shipped,
            payment_method: PaymentMethod::CreditCard,
            channel: SalesChannel::PopUp,
            total: 980.0,
            tags: tags(&["赠品"]),
            flagged: false,
        },
        Order {
            number: "DX-1043".to_string(),
            placed_on: date(2024, 7, 15),
            customer_name: "张伟".to_string(),
            customer_email: "zhangwei@example.com".to_string(),
            status: OrderStatus::Fulfilled,
            payment_status: PaymentStatus::Paid,
            fulfillment_status: FulfillmentStatus::Delivered,
            payment_method: PaymentMethod::WechatPay,
            channel: SalesChannel::Subscription,
            total: 652.0,
            tags: tags(&["VIP"]),
            flagged: false,
        },
        Order {
            number: "DX-1042".to_string(),
            placed_on: date(2024, 7, 13),
            customer_name: "李雷".to_string(),
            customer_email: "lilei@example.com".to_string(),
            status: OrderStatus::PendingPayment,
            payment_status: PaymentStatus::Pending,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            payment_method: PaymentMethod::BankTransfer,
            channel: SalesChannel::Wholesale,
            total: 3750.0,
            tags: tags(&["缺货", "加急"]),
            flagged: true,
        },
        Order {
            number: "DX-1041".to_string(),
            placed_on: date(2024, 7, 11),
            customer_name: "丁一".to_string(),
            customer_email: "dingyi@example.com".to_string(),
            status: OrderStatus::Processing,
            payment_status: PaymentStatus::Paid,
            fulfillment_status: FulfillmentStatus::Picking,
            payment_method: PaymentMethod::Alipay,
            channel: SalesChannel::OnlineStore,
            total: 512.8,
            tags: tags(&["赠品"]),
            flagged: false,
        },
        Order {
            number: "DX-1040".to_string(),
            placed_on: date(2024, 7, 10),
            customer_name: "Grace Li".to_string(),
            customer_email: "grace@example.com".to_string(),
            status: OrderStatus::Fulfilled,
            payment_status: PaymentStatus::Paid,
            fulfillment_status: FulfillmentStatus::Delivered,
            payment_method: PaymentMethod::CreditCard,
            channel: SalesChannel::Marketplace,
            total: 786.2,
            tags: tags(&["VIP", "赠品"]),
            flagged: false,
        },
        Order {
            number: "DX-1039".to_string(),
            placed_on: date(2024, 7, 9),
            customer_name: "赵敏".to_string(),
            customer_email: "zhaomin@example.com".to_string(),
            status: OrderStatus::Draft,
            payment_status: PaymentStatus::Pending,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            payment_method: PaymentMethod::Cash,
            channel: SalesChannel::PopUp,
            total: 210.0,
            tags: tags(&["需回访"]),
            flagged: false,
        },
    ]
}

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("valid mock date")
}

fn tags(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn initials(input: &str) -> String {
    let initials: String = input
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect();
    if initials.is_empty() {
        input.chars().take(2).collect()
    } else {
        initials.to_uppercase()
    }
}

#[component]
pub fn Orders() -> Element {
    let search = use_signal(|| String::new());
    let status_filter = use_signal(|| None::<OrderStatus>);
    let payment_filter = use_signal(|| None::<PaymentStatus>);
    let fulfillment_filter = use_signal(|| None::<FulfillmentStatus>);
    let channel_filter = use_signal(|| None::<SalesChannel>);
    let method_filter = use_signal(|| None::<PaymentMethod>);
    let tags_filter = use_signal(|| Vec::<String>::new());
    let min_total = use_signal(|| 0.0f32);
    let flagged_only = use_signal(|| false);
    let date_range = use_signal(|| None::<DateRange>);
    let pipeline = use_signal(|| vec!["all".to_string()]);
    let page = use_signal(|| 1usize);

    {
        let search_signal = search.clone();
        let status_signal = status_filter.clone();
        let payment_signal = payment_filter.clone();
        let fulfillment_signal = fulfillment_filter.clone();
        let channel_signal = channel_filter.clone();
        let method_signal = method_filter.clone();
        let tags_signal = tags_filter.clone();
        let range_signal = date_range.clone();
        let min_total_signal = min_total.clone();
        let flagged_signal = flagged_only.clone();
        let pipeline_signal = pipeline.clone();
        let mut page_signal = page.clone();

        use_effect(move || {
            search_signal();
            status_signal();
            payment_signal();
            fulfillment_signal();
            channel_signal();
            method_signal();
            tags_signal();
            range_signal();
            min_total_signal();
            flagged_signal();
            pipeline_signal();
            page_signal.set(1);
        });
    }

    let all_orders = seeded_orders();
    let total_orders_count = all_orders.len();

    let search_value = search();
    let search_term = search_value.to_lowercase();
    let status_selected = status_filter();
    let payment_selected = payment_filter();
    let fulfillment_selected = fulfillment_filter();
    let channel_selected = channel_filter();
    let method_selected = method_filter();
    let selected_tags = tags_filter();
    let active_tag_count = selected_tags.len();
    let min_total_value = min_total();
    let flagged_only_value = flagged_only();
    let date_range_selected = date_range();
    let date_range_label = date_range_selected
        .map(|range| {
            let start = range.start.format("%Y-%m-%d");
            let end = range.end.format("%Y-%m-%d");
            if range.start == range.end {
                format!("{}", start)
            } else {
                format!("{} → {}", start, end)
            }
        })
        .unwrap_or_else(|| "选择日期范围".to_string());
    let date_range_helper = date_range_selected
        .map(|range| {
            let span = (range.end - range.start).num_days().abs() + 1;
            format!("覆盖 {} 天，点击可修改", span)
        })
        .unwrap_or_else(|| "未限制下单日期".to_string());
    let pipeline_values = pipeline();
    let pipeline_value = pipeline_values
        .first()
        .cloned()
        .unwrap_or_else(|| "all".to_string());
    let tag_chip_options: Vec<CheckboxChipOption> = AVAILABLE_TAGS
        .iter()
        .map(|tag| CheckboxChipOption::new(*tag, *tag))
        .collect();

    let filtered: Vec<Order> = all_orders
        .iter()
        .cloned()
        .filter(|order| {
            let matches_search = if !search_term.is_empty() {
                let composite = format!(
                    "{} {} {} {} {}",
                    order.number,
                    order.customer_name,
                    order.customer_email,
                    order.channel.label(),
                    order.tags.join(" ")
                )
                .to_lowercase();
                composite.contains(&search_term)
            } else {
                true
            };

            let matches_status = status_selected
                .map(|expected| order.status == expected)
                .unwrap_or(true);
            let matches_payment = payment_selected
                .map(|expected| order.payment_status == expected)
                .unwrap_or(true);
            let matches_fulfillment = fulfillment_selected
                .map(|expected| order.fulfillment_status == expected)
                .unwrap_or(true);
            let matches_channel = channel_selected
                .map(|expected| order.channel == expected)
                .unwrap_or(true);
            let matches_method = method_selected
                .map(|expected| order.payment_method == expected)
                .unwrap_or(true);

            let matches_tags = if selected_tags.is_empty() {
                true
            } else {
                selected_tags
                    .iter()
                    .all(|tag| order.tags.iter().any(|candidate| candidate == tag))
            };

            let matches_range = if let Some(range) = date_range_selected {
                let date = order.placed_on;
                date >= range.start && date <= range.end
            } else {
                true
            };

            let matches_total = if min_total_value > 0.0 {
                order.total >= min_total_value
            } else {
                true
            };

            let matches_flagged = if flagged_only_value {
                order.flagged
            } else {
                true
            };

            let matches_pipeline = match pipeline_value.as_str() {
                "awaiting_fulfillment" => !matches!(
                    order.fulfillment_status,
                    FulfillmentStatus::Delivered | FulfillmentStatus::Returned
                ),
                "overdue" => matches!(order.payment_status, PaymentStatus::Overdue),
                "vip" => order.tags.iter().any(|tag| tag == "VIP"),
                _ => true,
            };

            matches_search
                && matches_status
                && matches_payment
                && matches_fulfillment
                && matches_channel
                && matches_method
                && matches_tags
                && matches_range
                && matches_total
                && matches_flagged
                && matches_pipeline
        })
        .collect();

    let filtered_total = filtered.len();
    let gross_revenue = filtered.iter().map(|order| order.total).sum::<f32>();
    let average_order_value = if filtered_total > 0 {
        gross_revenue / filtered_total as f32
    } else {
        0.0
    };
    let outstanding_payments = filtered
        .iter()
        .filter(|order| {
            matches!(
                order.payment_status,
                PaymentStatus::Pending | PaymentStatus::Overdue
            )
        })
        .count();
    let awaiting_fulfillment = filtered
        .iter()
        .filter(|order| {
            !matches!(
                order.fulfillment_status,
                FulfillmentStatus::Delivered | FulfillmentStatus::Returned
            )
        })
        .count();
    let flagged_orders = filtered.iter().filter(|order| order.flagged).count();

    let page_count = ((filtered_total + PAGE_SIZE - 1) / PAGE_SIZE).max(1);
    let active_page = page();
    let effective_page = active_page.max(1).min(page_count);
    if active_page != effective_page {
        let mut page_signal = page.clone();
        page_signal.set(effective_page);
    }
    let offset = effective_page.saturating_sub(1) * PAGE_SIZE;
    let paginated_orders: Vec<Order> = filtered
        .iter()
        .skip(offset)
        .take(PAGE_SIZE)
        .cloned()
        .collect();

    let status_options = OrderStatus::all()
        .iter()
        .map(|status| SelectOption::new(status.label(), status.key()))
        .collect::<Vec<_>>();
    let payment_options = PaymentStatus::all()
        .iter()
        .map(|status| SelectOption::new(status.label(), status.key()))
        .collect::<Vec<_>>();
    let fulfillment_options = FulfillmentStatus::all()
        .iter()
        .map(|status| SelectOption::new(status.label(), status.key()))
        .collect::<Vec<_>>();
    let channel_options = SalesChannel::all()
        .iter()
        .map(|channel| SelectOption::new(channel.label(), channel.key()))
        .collect::<Vec<_>>();
    let method_options = PaymentMethod::all()
        .iter()
        .map(|method| SelectOption::new(method.label(), method.key()))
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "ui-stack",
            style: "gap: 1.5rem;",
            Card {
                CardHeader {
                    CardTitle { "订单管理" }
                    CardDescription { "综合筛选订单、监控支付与履约状态，获取健康度指标。" }
                }
                CardContent {
                    div { class: "orders-metrics",
                        div { class: "orders-metric-card",
                            span { class: "orders-metric-label", "筛选后订单" }
                            span { class: "orders-metric-value", "{filtered_total}" }
                            span { class: "orders-metric-sub", {format!("共 {} 条记录", total_orders_count)} }
                        }
                        div { class: "orders-metric-card",
                            span { class: "orders-metric-label", "筛选总收入" }
                            span { class: "orders-metric-value", {format!("¥{:.0}", gross_revenue)} }
                            span { class: "orders-metric-sub", {format!("平均客单价 ¥{:.0}", average_order_value)} }
                        }
                        div { class: "orders-metric-card",
                            span { class: "orders-metric-label", "待处理支付" }
                            span { class: "orders-metric-value", "{outstanding_payments}" }
                            span { class: "orders-metric-sub", "包含逾期与待入账订单" }
                        }
                        div { class: "orders-metric-card",
                            span { class: "orders-metric-label", "履约队列" }
                            span { class: "orders-metric-value", "{awaiting_fulfillment}" }
                            span { class: "orders-metric-sub", {format!("标记关注 {} 单", flagged_orders)} }
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "筛选器" }
                    CardDescription { "组合多个维度快速圈定目标订单，可随时重置。" }
                }
                CardContent {
                    div { class: "orders-filter-grid",
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { html_for: "order-search", "关键词" }
                            Input {
                                id: Some("order-search".to_string()),
                                placeholder: Some("订单号 / 客户 / 邮箱".to_string()),
                                value: Some(search_value.clone()),
                                on_input: {
                                    let mut setter = search.clone();
                                    move |event: FormEvent| setter.set(event.value())
                                },
                            }
                        }
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { "订单状态" }
                            Select {
                                placeholder: "全部状态",
                                options: status_options.clone(),
                                selected: status_selected.map(|status| status.key().to_string()),
                                on_change: {
                                    let mut setter = status_filter.clone();
                                    move |value: String| setter.set(OrderStatus::from_key(&value))
                                },
                            }
                        }
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { "支付状态" }
                            Select {
                                placeholder: "全部支付".to_string(),
                                options: payment_options.clone(),
                                selected: payment_selected.map(|status| status.key().to_string()),
                                on_change: {
                                    let mut setter = payment_filter.clone();
                                    move |value: String| setter.set(PaymentStatus::from_key(&value))
                                },
                            }
                        }
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { "履约状态" }
                            Select {
                                placeholder: "全部履约".to_string(),
                                options: fulfillment_options.clone(),
                                selected: fulfillment_selected.map(|status| status.key().to_string()),
                                on_change: {
                                    let mut setter = fulfillment_filter.clone();
                                    move |value: String| setter.set(FulfillmentStatus::from_key(&value))
                                },
                            }
                        }
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { "销售渠道" }
                            Select {
                                placeholder: "全部渠道".to_string(),
                                options: channel_options.clone(),
                                selected: channel_selected.map(|channel| channel.key().to_string()),
                                on_change: {
                                    let mut setter = channel_filter.clone();
                                    move |value: String| setter.set(SalesChannel::from_key(&value))
                                },
                            }
                        }
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { "支付方式" }
                            Select {
                                placeholder: "全部方式".to_string(),
                                options: method_options.clone(),
                                selected: method_selected.map(|method| method.key().to_string()),
                                on_change: {
                                    let mut setter = method_filter.clone();
                                    move |value: String| setter.set(PaymentMethod::from_key(&value))
                                },
                            }
                        }
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { "下单日期" }
                            Popover {
                                placement: "bottom".to_string(),
                                trigger: rsx! {
                                    button {
                                        class: "orders-date-trigger",
                                        r#type: "button",
                                        span { class: "orders-date-trigger-text", "{date_range_label}" }
                                        span { class: "orders-date-trigger-icon", "📅" }
                                    }
                                },
                                content: rsx! {
                                    div { class: "orders-date-popover",
                                        DateRangePicker {
                                            value: date_range.clone(),
                                            on_change: {
                                                let mut setter = date_range.clone();
                                                move |range: Option<DateRange>| setter.set(range)
                                            },
                                        }
                                    }
                                },
                            }
                            span { class: "ui-field-helper", "{date_range_helper}" }
                        }
                        div { class: "ui-stack", style: "gap: 0.5rem;",
                            Label { "订单金额 (¥)" }
                            Slider {
                                value: min_total_value,
                                min: 0.0,
                                max: 4000.0,
                                step: 50.0,
                                on_value_change: {
                                    let mut setter = min_total.clone();
                                    move |value: f32| setter.set(value)
                                },
                            }
                            span { class: "ui-field-helper", {format!("最低金额：¥{}", min_total_value as i32)} }
                        }
                        div { class: "ui-stack orders-filter-wide", style: "gap: 0.5rem;",
                            CheckboxChipGroup {
                                label: "标签",
                                values: tags_filter.clone(),
                                options: tag_chip_options.clone(),
                            }
                            span { class: "ui-field-helper", {format!("已选标签：{}", active_tag_count)} }
                        }
                    }
                }
                CardFooter {
                    div { class: "ui-cluster", style: "gap: 0.75rem;",
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            on_click: {
                                let mut setter_search = search.clone();
                                let mut setter_status = status_filter.clone();
                                let mut setter_payment = payment_filter.clone();
                                let mut setter_fulfillment = fulfillment_filter.clone();
                                let mut setter_channel = channel_filter.clone();
                                let mut setter_method = method_filter.clone();
                                let mut setter_tags = tags_filter.clone();
                                let mut setter_range = date_range.clone();
                                let mut setter_min_total = min_total.clone();
                                let mut setter_flagged = flagged_only.clone();
                                let mut setter_pipeline = pipeline.clone();
                                let mut setter_page = page.clone();
                                move |_| {
                                    setter_search.set(String::new());
                                    setter_status.set(None);
                                    setter_payment.set(None);
                                    setter_fulfillment.set(None);
                                    setter_channel.set(None);
                                    setter_method.set(None);
                                    setter_tags.set(Vec::new());
                                    setter_range.set(None);
                                    setter_min_total.set(0.0);
                                    setter_flagged.set(false);
                                    setter_pipeline.set(vec!["all".to_string()]);
                                    setter_page.set(1);
                                }
                            },
                            "重置筛选"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            "导出报表"
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "订单列表" }
                    CardDescription { "结果会实时反映筛选条件，可分页浏览。" }
                }
                CardContent {
                    div { class: "ui-stack", style: "gap: 1rem;",
                        if paginated_orders.is_empty() {
                            div { class: "orders-empty",
                                span { class: "orders-metric-label", "没有匹配的订单" }
                                span { class: "orders-metric-sub", "调整筛选条件或清除限制重新查看。" }
                            }
                        } else {
                            Table {
                                TableHeader {
                                    TableRow {
                                        TableHead { "订单信息" }
                                        TableHead { "日期" }
                                        TableHead { "状态" }
                                        TableHead { "支付" }
                                        TableHead { "履约" }
                                        TableHead { "渠道" }
                                        TableHead { "金额" }
                                        TableHead { "标签" }
                                        TableHead { "操作" }
                                    }
                                }
                                TableBody {
                                    for order in paginated_orders.iter().cloned() {
                                        {
                                            let badge_variant = order.status.badge();
                                            let initials = initials(&order.customer_name);
                                            let date_display = order.placed_on.format("%Y-%m-%d").to_string();
                                            rsx! {
                                                TableRow {
                                                    TableCell {
                                                        div { style: "display: flex; align-items: center; gap: 0.75rem;",
                                                            Avatar {
                                                                fallback: Some(initials.clone()),
                                                                alt: Some(order.customer_name.clone()),
                                                            }
                                                            div { style: "display: flex; flex-direction: column; gap: 0.25rem;",
                                                                span { style: "font-weight: 600;", "{order.customer_name}" }
                                                                span { class: "ui-field-helper", "{order.customer_email}" }
                                                            }
                                                        }
                                                    }
                                                    TableCell { "{date_display}" }
                                                    TableCell {
                                                        Badge { variant: badge_variant, "{order.status.label()}" }
                                                    }
                                                    TableCell {
                                                        Badge {
                                                            variant: match order.payment_status {
                                                                PaymentStatus::Paid => BadgeVariant::Secondary,
                                                                PaymentStatus::Refunded => BadgeVariant::Outline,
                                                                PaymentStatus::Overdue => BadgeVariant::Destructive,
                                                                PaymentStatus::Pending => BadgeVariant::Default,
                                                            },
                                                            "{order.payment_status.label()}"
                                                        }
                                                    }
                                                    TableCell { "{order.fulfillment_status.label()}" }
                                                    TableCell { "{order.channel.label()}" }
                                                    TableCell { {format!("¥{:.2}", order.total)} }
                                                    TableCell {
                                                        div { class: "orders-tag-cloud",
                                                            for tag in order.tags.iter().cloned() {
                                                                {
                                                                    rsx! { Badge { variant: BadgeVariant::Outline, "{tag}" } }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    TableCell {
                                                        Button {
                                                            variant: ButtonVariant::Ghost,
                                                            size: ButtonSize::Sm,
                                                            "查看"
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
                }
                CardFooter {
                    if filtered_total > PAGE_SIZE {
                        Pagination {
                            total_pages: page_count,
                            current_page: effective_page,
                            on_page_change: {
                                let mut setter = page.clone();
                                move |page_index: usize| setter.set(page_index)
                            },
                        }
                    }
                }
            }
        }
    }
}
