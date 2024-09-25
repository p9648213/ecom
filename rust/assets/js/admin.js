const admin_dashboard = document.getElementById("admin-dashboard");
const admin_products = document.getElementById("admin-products");
const admin_orders = document.getElementById("admin-orders");
const product_image_upload = document.getElementById("product-image-upload");
const product_drag_and_drop = document.getElementById("product-drag-and-drop");
const product_image_loaded = document.getElementById("product-image-loaded");

function toggleDrawerBackground(pathname = window.location.pathname) {
  if (pathname === "/admin/dashboard") {
    admin_dashboard.classList.add("bg-muted");
    admin_products.classList.remove("bg-muted");
    admin_orders.classList.remove("bg-muted");
  } else if (pathname === "/admin/products") {
    admin_dashboard.classList.remove("bg-muted");
    admin_products.classList.add("bg-muted");
    admin_orders.classList.remove("bg-muted");
  } else if (pathname === "/admin/orders") {
    admin_dashboard.classList.remove("bg-muted");
    admin_products.classList.remove("bg-muted");
    admin_orders.classList.add("bg-muted");
  }
}

document.addEventListener("DOMContentLoaded", () => {
  toggleDrawerBackground();
});

document.addEventListener("htmx:pushedIntoHistory", () => {
  toggleDrawerBackground();
});

product_image_loaded.querySelector("button").addEventListener("click", () => {
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
