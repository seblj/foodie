use anyhow::anyhow;
use leptos::*;
use std::time::Duration;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Warning,
    Error,
}

#[component]
pub fn Toaster(children: Children) -> impl IntoView {
    let (toaster, set_toaster) = create_signal(Toaster::new());
    let (removed, set_removed) = create_signal(vec![]);
    provide_context(set_toaster);

    let t = move || {
        toaster()
            .alerts
            .iter()
            .map(|a| {
                let toast = &a.0;
                if let Some(timeout) = toast.timeout {
                    let id = a.1;
                    if !removed().contains(&id) {
                        set_timeout(move || set_toaster.update(|t| t.remove(id)), timeout);
                        set_removed.update(|v| v.push(id));
                    }
                }
                toast.get()
            })
            .collect::<Vec<_>>()
    };

    view! {
        <>
            <div class="toast toast-end w-80">{t}</div>
            {children()}
        </>
    }
}

#[derive(Clone)]
pub struct Toast {
    pub ty: ToastType,
    pub body: String,
    pub timeout: Option<Duration>,
}

impl Toast {
    pub fn get(&self) -> impl IntoView {
        let (icon, alert_type) = match self.ty {
            ToastType::Success => ("M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z", "alert-success"),
            ToastType::Warning => ("M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z", "alert-warning"),
            ToastType::Error => ("M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z", "alert-error"),
        };

        view! {
            <div role="alert" class=format!("alert {}", alert_type)>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-current shrink-0 h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d=icon
                    ></path>
                </svg>
                <span>{self.body.to_string()}</span>
            </div>
        }
    }
}

#[derive(Clone)]
pub struct Toaster {
    pub alerts: Vec<(Toast, i32)>,
    last_id: i32,
}

impl Toaster {
    pub fn new() -> Self {
        Toaster {
            alerts: vec![],
            last_id: 0,
        }
    }

    fn remove(&mut self, id: i32) {
        self.alerts.retain(|it| it.1 != id);
    }

    pub fn add(&mut self, toast: Toast) {
        self.last_id += 1;
        self.alerts.push((toast, self.last_id));
    }
}

pub trait ToasterTrait {
    fn add(&self, toast: Toast);
}

impl ToasterTrait for WriteSignal<Toaster> {
    fn add(&self, toast: Toast) {
        self.update(|t| t.add(toast))
    }
}

pub fn use_toast() -> Result<WriteSignal<Toaster>, anyhow::Error> {
    use_context::<WriteSignal<Toaster>>().ok_or_else(|| anyhow!("Couldn't find context"))
}
