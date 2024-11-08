use maud::{html, Markup};

use crate::views::pages::admin::{admin_header, admin_sidebar};

pub fn create_admin_layout(content: Markup) -> Markup {
    html! {
      script src="/assets/js/drawer.js" defer type="module" {};
      script src="/assets/js/admin.js" defer type="module" {};
      script src="/assets/js/admin_product.js" defer type="module" {};
      title { "Admin" }
      div class="flex w-full min-h-screen" {
        (admin_sidebar())
        div class="flex flex-col flex-1" {
          (admin_header())
          main class="flex flex-col flex-1 bg-muted/40 p-4 md:p-6" {
            (content)
          }
        }
      }
    }
}
