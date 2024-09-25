use maud::{html, Markup};

use crate::views::pages::admin::SelectOption;

pub fn select(label: &str, name: &str, id: &str, option: &[SelectOption]) -> Markup {
    html! {
      div class="grid w-full gap-1.5" {
        label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" {
          (label)
        }
        select id=(id) name=(name) class="mt-2 block w-full rounded-md py-1.5 pl-3 pr-10 text-gray-900 sm:text-sm sm:leading-6" {
          @for option in option {
            option value=(option.value) {
              (option.label)
            }
          }
        }
      }
    }
}
