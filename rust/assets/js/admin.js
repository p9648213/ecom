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

function resetAddProductForm() {
  const addProductHeader = document.getElementById("products-drawer-header");

  if (addProductHeader && !addProductHeader.textContent.includes("Edit")) {
    return;
  }

  const addProductForm = document.getElementById("add-product-form");
  const addProductButton = addProductForm.querySelector(
    'button[type="submit"]'
  );

  Array.from(addProductForm.elements).forEach((element) => {
    if (element.id === "add-product-brand") {
      element.value = "adidas";
    } else if (element.id === "add-product-category") {
      element.value = "men";
    } else {
      element.value = "";
    }
  });

  addProductHeader.textContent = "Add New Product";
  addProductButton.textContent = "Add";
  addProductForm.setAttribute("hx-post", "/admin/products/add");
  addProductForm.setAttribute("hx-target", "#admin-product-list");
  addProductForm.setAttribute("hx-swap", "afterbegin");
  addProductForm.setAttribute("enctype", "multipart/form-data");
  htmx.process(addProductForm);
}

document.addEventListener("htmx:afterRequest", (event) => {
  if (
    event.detail.pathInfo.requestPath === "/admin/products/add" &&
    event.detail.successful === true
  ) {
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape" }));
  }
});

window.resetAddProductForm = resetAddProductForm;
