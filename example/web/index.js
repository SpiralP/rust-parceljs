let message;
if (process.env.NODE_ENV !== "production") {
  message = "HELLO DEBUG!";
} else {
  message = "HELLO RELEASE!";
}

const el = document.createElement("h1");
el.innerText = message;
document.body.appendChild(el);
