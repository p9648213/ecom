use maud::{html, Markup};

use crate::{middleware::auth::UserExtention, views::pages::shop::shop_header};

pub fn create_shop_layout(content: Markup, user_extention: UserExtention) -> Markup {
    html! {
      script src="/assets/js/shop.js" defer {};
      title {"Shopping"}
      (shop_header(user_extention))
      (content)
    }
}
