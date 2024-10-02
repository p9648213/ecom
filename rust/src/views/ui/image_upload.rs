use maud::{html, Markup};

use crate::views::ui::icons::{file_icon, upload_icon, x_icon};
pub fn image_upload(name: &str) -> Markup {
    html! {
      div class="w-full max-w-md mx-auto mt-4" {
        label class="font-semibold mb-2 block" {
          "Upload Image"
        }
        div class="border-2 border-dashed rounded-lg p-4" {
          input name=(name) id="product-image-upload" type="file" class="hidden" {}
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
              button id="clear-image-button" {
                (x_icon())
              }
            }
          }
        }
      }
    }
}
