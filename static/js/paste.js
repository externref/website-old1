function savePaste() {
  var _a;
  var pasteData =
    (_a = document.getElementById("paste_body")) === null || _a === void 0
      ? void 0
      : _a.innerText;
  switch (pasteData) {
    case undefined:
      return alert("Internal error.");
    case "":
      return alert("Cannot create an empty paste.");
    default: {
      window.location.replace(
        "./paste/create_paste?paste_data=".concat(encodeURIComponent(pasteData))
      );
    }
  }
}
