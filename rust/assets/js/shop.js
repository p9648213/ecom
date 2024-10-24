// Get the dropdown, its container, and the "Logged in as" div
const dropdownContainer = document.getElementById("user-dropdown-container");
const dropdown = document.getElementById("user-dropdown");
const loggedInDiv = document.getElementById("user-logged-in");

// Function to toggle the dropdown visibility
const toggleDropdown = () => {
  dropdown.classList.toggle("hidden");
};

// Function to hide the dropdown if clicking outside
const handleClickOutside = (event) => {
  if (!dropdownContainer.contains(event.target)) {
    dropdown.classList.add("hidden");
  }
};

if (dropdownContainer) {
  // Add event listener to the dropdown container for toggle
  dropdownContainer.addEventListener("click", (event) => {
    event.stopPropagation();
    toggleDropdown();
  });
}

if (loggedInDiv) {
  // Prevent hiding the dropdown when clicking on the "Logged in as" div
  loggedInDiv.addEventListener("click", (event) => {
    event.stopPropagation();
  });
}

// Add event listener for outside clicks
if (dropdownContainer && loggedInDiv && dropdown) {
  document.addEventListener("click", handleClickOutside);
}
