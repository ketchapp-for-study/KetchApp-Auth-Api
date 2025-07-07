use leptos::prelude::{create_signal, signal, ElementChild, Signal};
use leptos::*;
use gloo_timers::future::TimeoutFuture;
use std::sync::OnceLock;
use leptos::prelude::{For, Get, RwSignal, Set, StyleAttribute};
use leptos::reactive::spawn_local;
use web_sys::js_sys;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug)]
pub struct Notification {
    pub id: u64,
    pub message: String,
    pub kind: NotificationType,
}

#[component]
fn NotificationBar(progress: Signal<f32>, bar_color: &'static str) -> impl IntoView {
    view! {
        <div style=move || format!(
            "position: absolute; left: 0; bottom: 0; height: 4px; background: {}; border-radius: 0 0 0.4rem 0.4rem; width: {}%; transition: width 0.1s linear;",
            bar_color,
            progress.get() * 100.0
        )></div>
    }
}

static NOTIFICATIONS: OnceLock<RwSignal<Vec<Notification>>> = OnceLock::new();

fn notifications_signal() -> &'static RwSignal<Vec<Notification>> {
    NOTIFICATIONS.get_or_init(|| RwSignal::new(Vec::new()))
}

pub fn notify(message: &str, kind: NotificationType) {
    let notifications = notifications_signal();
    let mut current = notifications.get();
    let id = js_sys::Date::now() as u64;
    current.push(Notification {
        id,
        message: message.to_string(),
        kind,
    });
    notifications.set(current.clone());
    // Remove after 3 seconds
    let notifications = notifications.clone();
    let id_copy = id;
    spawn_local(async move {
        TimeoutFuture::new(3200).await;
        let mut current = notifications.get();
        current.retain(|n| n.id != id_copy);
        notifications.set(current);
    });
}

#[component]
pub fn NotificationView() -> impl IntoView {
    let notifications = notifications_signal().clone();
    view! {
        <div style="position: fixed; top: 1rem; right: 1rem; z-index: 99999; display: flex; flex-direction: column; gap: 0.5rem; pointer-events: none;">
            <For each=move || notifications.get() key=|n| n.id let:notification>
                {
                    let (progress, set_progress) = create_signal(1.0f32);
                    spawn_local({
                        let set_progress = set_progress.clone();
                        async move {
                            let steps = 30;
                            for i in 0..steps {
                                TimeoutFuture::new(100).await;
                                set_progress.set(1.0 - (i as f32 / steps as f32));
                            }
                        }
                    });
                    let (bg, bar) = match notification.kind {
                        NotificationType::Info => ("#23272a", "#6975f8"),
                        NotificationType::Warn => ("#f8c967", "#f8a500"),
                        NotificationType::Error => ("#f86c6b", "#b91d1d"),
                    };
                    view! {
                        <div style=format!("background: {}; color: #fff; padding: 1rem 2rem; border-radius: 0.4rem; box-shadow: 0 2px 8px rgba(0,0,0,0.2); min-width: 200px; position: relative; overflow: hidden; z-index: 99999; pointer-events: auto; font-size: 1.1em; font-weight: 700; letter-spacing: 0.01em;", bg)>
                            {notification.message.clone()}
                            <NotificationBar progress=Signal::from(progress) bar_color=bar />
                        </div>
                    }
                }
            </For>
        </div>
    }
}
