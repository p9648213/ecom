const admin_dashboard = document.getElementById("admin-dashboard");
const admin_products = document.getElementById("admin-products");
const admin_orders = document.getElementById("admin-orders");

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
