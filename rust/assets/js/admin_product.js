function admin_product_load_event() {
  const product_image_upload = document.getElementById("product-image-upload");
  const product_drag_and_drop = document.getElementById(
    "product-drag-and-drop"
  );
  const product_image_loaded = document.getElementById("product-image-loaded");
  const clear_image_button = document.getElementById("clear-image-button");

  clear_image_button.addEventListener("click", () => {
    product_image_upload.value = "";
    product_drag_and_drop.classList.remove("hidden");
    product_image_loaded.classList.add("hidden");
  });

  product_image_upload.addEventListener("change", (event) => {
    const file = event.target.files[0];
    if (file) {
      product_drag_and_drop.classList.add("hidden");
      product_image_loaded.classList.remove("hidden");
      product_image_loaded.querySelector("p").textContent = file.name;
    }
  });
}

if (window.location.pathname === "/admin/products") {
  admin_product_load_event();
}

window.addEventListener("htmx:afterSettle", (_) => {
  if (window.location.pathname === "/admin/products") {
    admin_product_load_event();
  }
});
