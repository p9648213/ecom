use maud::{html, Markup};

use crate::views::pages::admin::{admin_header, admin_sidebar};

pub fn create_admin_layout(content: Markup) -> Markup {
    html! {
      script src="/assets/js/drawer.js" defer type="module" {};
      script src="/assets/js/admin.js" defer type="module" {};
      script src="/assets/js/admin_product.js" defer type="module" {};
      title { "Admin" }
      div class="flex min-h-screen w-full" {
        (admin_sidebar())
        div class="flex flex-1 flex-col" {
          (admin_header())
          main class="flex-1 flex bg-muted/40 p-4 md:p-6" {
            (content)
          }
        }
      }
    }
}
