use maud::{html, Markup};

pub fn upload_icon() -> Markup {
    html! {
      svg class="w-10 h-10 text-muted-foreground mb-2" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="24" height="24" stroke-linecap="round" {
        path d="M12 13v8" {}
        path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" {}
        path d="m8 17 4-4 4 4" {}
      }
    }
}

pub fn x_icon() -> Markup {
    html! {
          svg class="w-4 h-4" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="24" height="24" stroke-linecap="round" {
            path d="M18 6 6 18" {}
            path d="m6 6 12 12" {}
        }
    }
}

pub fn file_icon() -> Markup {
    html! {
      svg class="w-8 text-primary mr-2 h-8" stroke="currentColor" stroke-width="2" width="24" viewBox="0 0 24 24" stroke-linecap="round" fill="none" stroke-linejoin="round" height="24" xmlns="http://www.w3.org/2000/svg" {
        path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" {}
        path d="M14 2v4a2 2 0 0 0 2 2h4" {}
      }
    }
}

pub fn loading_icon() -> Markup {
    html! {
        svg class="animate-spin h-7 w-7" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" {
            circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" {
            }
            path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" {
            }
        }
    }
}

pub fn align_justify_icon() -> Markup {
    html! {
        svg class="lucide lucide-align-justify" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            line x1="3" x2="21" y1="6" y2="6" {
            }
            line x1="3" x2="21" y1="12" y2="12" {
            }
            line x1="3" x2="21" y1="18" y2="18" {
            }
        }
    }
}

pub fn log_out_icon() -> Markup {
    html! {
        svg class="lucide lucide-log-out" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" {
            }
            polyline points="16 17 21 12 16 7" {
            }
            line x1="21" x2="9" y1="12" y2="12" {
            }
        }
    }
}

pub fn log_out_icon_small() -> Markup {
    html! {
        svg class="lucide lucide-log-out" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" {
            }
            polyline points="16 17 21 12 16 7" {
            }
            line x1="21" x2="9" y1="12" y2="12" {
            }
        }
    }
}

pub fn chart_no_axes_combined_icon() -> Markup {
    html! {
        svg class="lucide lucide-chart-no-axes-combined" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M12 16v5" {
            }
            path d="M16 14v7" {
            }
            path d="M20 10v11" {
            }
            path d="m22 3-8.646 8.646a.5.5 0 0 1-.708 0L9.354 8.354a.5.5 0 0 0-.707 0L2 15" {
            }
            path d="M4 18v3" {
            }
            path d="M8 14v7" {
            }
        }
    }
}

pub fn badge_check_icon() -> Markup {
    html! {
        svg class="lucide lucide-badge-check" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" {
            }
            path d="m9 12 2 2 4-4" {
            }
        }
    }
}

pub fn layout_dashboard_icon() -> Markup {
    html! {
        svg class="lucide lucide-layout-dashboard" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            rect width="7" height="9" x="3" y="3" rx="1" {
            }
            rect width="7" height="5" x="14" y="3" rx="1" {
            }
            rect width="7" height="9" x="14" y="12" rx="1" {
            }
            rect width="7" height="5" x="3" y="16" rx="1" {
            }
        }
    }
}

pub fn shopping_basket_icon() -> Markup {
    html! {
        svg class="lucide lucide-shopping-basket" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="m15 11-1 9" {
            }
            path d="m19 11-4-7" {
            }
            path d="M2 11h20" {
            }
            path d="m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8a2 2 0 0 0 2-1.6l1.7-7.4" {
            }
            path d="M4.5 15.5h15" {
            }
            path d="m5 11 4-7" {
            }
            path d="m9 11 1 9" {
            }
        }
    }
}

pub fn house_plug_icon() -> Markup {
    html! {
        svg."lucide lucide-house-plug" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="24" height="24" stroke-linecap="round" {
            path d="M10 12V8.964" {}
            path d="M14 12V8.964" {}
            path d="M15 12a1 1 0 0 1 1 1v2a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2v-2a1 1 0 0 1 1-1z" {}
            path d="M8.5 21H5a2 2 0 0 1-2-2v-9a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2h-5a2 2 0 0 1-2-2v-2" {}
        }
    }
}

pub fn shopping_cart_icon() -> Markup {
    html! {
        svg."lucide lucide-shopping-cart" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="22" height="22" stroke-linecap="round" {
            circle r="1" cy="21" cx="8" {}
            circle r="1" cy="21" cx="19" {}
            path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" {}
        }
    }
}

pub fn user_icon() -> Markup {
    html! {
        svg."lucide lucide-user" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="20" height="20" stroke-linecap="round" {
            path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" {}
            circle r="4" cy="7" cx="12" {}
        }
    }
}

pub fn chevron_left_icon() -> Markup {
    html! {
        svg."lucide lucide-chevron-left" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="20" height="20" stroke-linecap="round" {
            path d="m15 18-6-6 6-6" {}
        }
    }
}
