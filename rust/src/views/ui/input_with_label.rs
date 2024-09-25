use maud::{html, Markup};

pub fn input_with_label(
    label: &str,
    input_type: &str,
    name: &str,
    id: &str,
    placeholder: &str,
) -> Markup {
    html! {
      div class="grid w-full gap-1.5" {
          label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" {
              (label)
          }
          div class="mt-2" {
              input type=(input_type) name=(name) id=(id) autocomplete="on" class="flex h-10 w-full rounded-md bg-background px-3 py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50" placeholder=(placeholder) {}
          }
      }
    }
}
