use maud::{html, Markup};

pub fn text_area(label: &str, name: &str, id: &str, placeholder: &str) -> Markup {
    html! {
      div class="grid w-full gap-1.5" {
        label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" {
          (label)
        }
        textarea
          name=(name)
          id=(id)
          placeholder=(placeholder)
          class="mt-2 flex min-h-[80px] w-full rounded-md bg-background px-3 py-2 text-sm  placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50" {}
      }
    }
}
