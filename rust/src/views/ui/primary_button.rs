use maud::{html, Markup};
use tailwind_fuse::tw_merge;

pub fn primary_button(text: Option<&str>, class: Option<&str>, icon: Option<Markup>) -> Markup {
    let default_button_class = String::from("inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outlin mt-2e-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2");

    let button_class = match class {
        Some(class) => {
            tw_merge!(default_button_class, class)
        }
        None => default_button_class.to_string(),
    };

    let button_text = match text {
        Some(text) => {
            html! {
              div { (text) }
            }
        }
        None => html! {},
    };

    let icon = match icon {
        Some(icon) => icon,
        None => html! {},
    };

    html! {
      button type="submit" class=(button_class) {
          (icon)
          (button_text)
      }
    }
}
