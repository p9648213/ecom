function toast({ message = "", type = "info", duration = 3000 }) {
  const main = document.getElementById("toast");

  if (main) {
    if(main.childNodes.length > 0){
      main.innerHTML="";
    }

    const toast = document.createElement("div");

    const icons = {
      success: "🎉",
      info: "🔧",
      warning: "⚠️",
      error: "💥"
    };
    const icon = icons[type];
    const delay = (duration / 1000).toFixed(2);

    toast.classList.add("toast", `toast--${type}`);
    toast.style.animation = `slideInLeft ease .3s, fadeOut linear 1s ${delay}s forwards`;

    toast.innerHTML = `
      <div class="toast__icon">
          ${icon}
      </div>
      <div class="toast__body">
          <p class="toast__msg">${message}</p>
      </div>
    `;

    main.appendChild(toast);
  }
}

document.body.addEventListener("toastmessage", function (event) {  
  if (event?.detail?.type === "success") {
    toast({
      message: event?.detail?.message,
      type: "success",
    });
  }
});

document.body.addEventListener("htmx:afterRequest", function (event) {
  const loginLinkEl = document.getElementById("login-link");
  const registerLinkEl = document.getElementById("register-link");

  if(loginLinkEl){
    loginLinkEl.classList.remove("disable-link");
  }

  if(registerLinkEl){
    registerLinkEl.classList.remove("disable-link");
  }

  if(event?.detail?.failed && event?.detail?.xhr?.responseText){
    toast({
      message: event?.detail?.xhr?.responseText,
      type: "error",
    });
  }
})

document.body.addEventListener("htmx:beforeRequest", function (_) {
  const loginLinkEl = document.getElementById("login-link");
  const registerLinkEl = document.getElementById("register-link");

  if(loginLinkEl){
    loginLinkEl.classList.add("disable-link");
  }

  if(registerLinkEl){
    registerLinkEl.classList.add("disable-link");
  }
})
