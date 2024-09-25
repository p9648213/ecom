use maud::{html, Markup};
pub fn image_upload() -> Markup {
    html! {
      div class="w-full max-w-md mx-auto mt-4" {
        label class="text-lg font-semibold mb-2 block" {
          "Upload Image"
        }
        div class="border-2 border-dashed rounded-lg p-4" {
          input id="product-image-upload" type="file" class="hidden" {}
          div id="product-drag-and-drop" {
            label for="product-image-upload" class="flex flex-col items-center justify-center h-32 cursor-pointer" {
              (upload_icon())
              span { "Drag & drop or click to upload image" }
            }
          }
          div id="product-image-loaded" class="hidden" {
            div class="flex items-center justify-between" {
              div class="flex items-center" {
                (file_icon())
              }
              p class="text-sm font-medium" {}
              button {
                (x_icon())
              }
            }
          }
        }
      }
    }
}

fn upload_icon() -> Markup {
    html! {
      svg class="w-10 h-10 text-muted-foreground mb-2" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="24" height="24" stroke-linecap="round" {
        path d="M12 13v8" {}
        path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" {}
        path d="m8 17 4-4 4 4" {}
      }
    }
}

fn x_icon() -> Markup {
    html! {
          svg class="w-4 h-4" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" width="24" height="24" stroke-linecap="round" {
            path d="M18 6 6 18" {}
            path d="m6 6 12 12" {}
        }
    }
}

fn file_icon() -> Markup {
    html! {
      svg class="w-8 text-primary mr-2 h-8" stroke="currentColor" stroke-width="2" width="24" viewBox="0 0 24 24" stroke-linecap="round" fill="none" stroke-linejoin="round" height="24" xmlns="http://www.w3.org/2000/svg" {
        path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" {}
        path d="M14 2v4a2 2 0 0 0 2 2h4" {}
      }
    }
}
