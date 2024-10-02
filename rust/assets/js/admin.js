const admin_dashboard = document.getElementById("admin-dashboard");
const admin_products = document.getElementById("admin-products");
const admin_orders = document.getElementById("admin-orders");
const drawer_products = document.getElementById("drawer-admin-products");
const drawer_orders = document.getElementById("drawer-admin-orders");
const drawer_dashboard = document.getElementById("drawer-admin-dashboard");

function toggleMenuItemsBackground(pathname = window.location.pathname) {
  if (pathname === "/admin/dashboard") {
    admin_dashboard.classList.add("bg-muted");
    admin_products.classList.remove("bg-muted");
    admin_orders.classList.remove("bg-muted");
    drawer_dashboard.classList.add("bg-muted");
    drawer_products.classList.remove("bg-muted");
    drawer_orders.classList.remove("bg-muted");
  } else if (pathname === "/admin/products") {
    admin_dashboard.classList.remove("bg-muted");
    admin_products.classList.add("bg-muted");
    admin_orders.classList.remove("bg-muted");
    drawer_dashboard.classList.remove("bg-muted");
    drawer_products.classList.add("bg-muted");
    drawer_orders.classList.remove("bg-muted");
  } else if (pathname === "/admin/orders") {
    admin_dashboard.classList.remove("bg-muted");
    admin_products.classList.remove("bg-muted");
    admin_orders.classList.add("bg-muted");
    drawer_dashboard.classList.remove("bg-muted");
    drawer_products.classList.remove("bg-muted");
    drawer_orders.classList.add("bg-muted");
  }
}

toggleMenuItemsBackground();

document.addEventListener("htmx:pushedIntoHistory", () => {
  toggleMenuItemsBackground();
});
