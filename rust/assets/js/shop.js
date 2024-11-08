// Helper function to set up a dropdown with click outside to close functionality
const setupDropdown = (containerId, dropdownId) => {
  const container = document.getElementById(containerId);
  const dropdown = document.getElementById(dropdownId);

  if (!container || !dropdown) return;

  const toggleDropdown = () => {
    dropdown.classList.toggle("hidden");
  };

  const handleClickOutside = (event) => {
    if (!container.contains(event.target)) {
      dropdown.classList.add("hidden");
    }
  };

  container.addEventListener("click", (event) => {
    event.stopPropagation();
    toggleDropdown();
  });

  if (containerId !== "user-dropdown-container") {
    dropdown.addEventListener("click", (event) => {
      event.stopPropagation();
    });

    dropdown.querySelector("div").childNodes.forEach((el) => {
      el.addEventListener("click", () => {
        toggleDropdown();
      });
    });
  }

  document.addEventListener("click", handleClickOutside);
};

function setupUserDropdown() {
  setupDropdown("user-dropdown-container", "user-dropdown");
  const loggedInDiv = document.getElementById("user-logged-in");
  if (loggedInDiv) {
    loggedInDiv.addEventListener("click", (event) => {
      event.stopPropagation();
    });
  }
}

function setupSortbyDropdown() {
  setupDropdown("sortby-dropdown-container", "sortby-dropdown");
}

// Init user and sort Dropdown
setupUserDropdown();

if (window.location.pathname === "/shop/listing") {
  setupSortbyDropdown();
}

let reInitUserDropdown = false;

document.body.addEventListener("htmx:afterSettle", (event) => {
  if (event?.detail?.pathInfo?.requestPath?.includes("/shop/")) {
    if (reInitUserDropdown === true) {
      setupUserDropdown();
      reInitUserDropdown = false;
    }
  } else {
    reInitUserDropdown = true;
  }

  if (event?.detail?.pathInfo?.requestPath.includes("/shop/listing")) {
    setupSortbyDropdown();
  }
});
