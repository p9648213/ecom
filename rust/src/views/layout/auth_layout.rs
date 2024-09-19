use maud::{html, Markup};

pub fn create_auth_layout(content: Markup) -> Markup {
    html! {
      div class="flex min-h-screen w-full bg-black" {
        div class="hidden lg:flex items-center justify-center bg-black w-1/2 px-12" {
          div class="max-w-md space-y-6 text-center text-primary-foreground" {
            h1 class="text-4xl font-extrabold tracking-tight"
            { "Welcome to Ecommerce Shopping" }
          }
        }
        div class="flex flex-1 items-center justify-center bg-background px-4 py-12 sm:px-6 lg:px-8" {
          (content)
        }
      }
    }
}
