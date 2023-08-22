#![allow(dead_code)]
use leptos::*;
use std::time::Duration;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct Toast {
    pub r#type: ToastType,
    pub body: String,
    pub timeout: Option<Duration>,
}

impl Toast {
    pub fn get(&self) -> impl IntoView {
        match self.r#type {
            ToastType::Error => {
                view! {
                    <div class="alert alert-danger" role="alert">
                        "Error"
                    </div>
                }
            }
            ToastType::Warning => {
                view! {
                    <div class="alert alert-warning" role="alert">
                        "Warning"
                    </div>
                }
            }
            ToastType::Success => {
                view! {
                    <div class="alert alert-success" role="alert">
                        "Success"
                    </div>
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Toaster {
    pub alerts: Vec<Toast>,
}

impl Toaster {
    pub fn new() -> Self {
        Toaster { alerts: vec![] }
    }

    pub fn add(&mut self, toast: Toast) {
        self.alerts.push(toast);
    }
}
