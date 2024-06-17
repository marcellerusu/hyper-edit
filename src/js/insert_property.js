function input() {
  let elem = document.createElement("div");
  elem.classList.add("input");
  elem.contentEditable = true;
  elem.placeholder = "insert property...";
  elem.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      fetch(`${location.pathname}`, {
        method: "POST",
        body: e.target.innerText,
      }).finally((_) => (location.search = ""));
    } else if (e.key === "Escape") {
      elem.blur();
    }
  });
  elem.addEventListener("blur", (_) => (elem.innerText = ""));
  return elem;
}

document.addEventListener("DOMContentLoaded", () => {
  let properties = document.querySelector(
    "[data-kind=rule] > [data-attr=properties]",
  );

  properties.append(input());
});
