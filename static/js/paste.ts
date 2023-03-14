function savePaste(): void {
  let pasteData: string | undefined =
    document.getElementById("paste_body")?.innerText;
  switch (pasteData) {
    case undefined:
      return alert("Internal error.");
    case "":
      return alert("Cannot create an empty paste.");
    default: {
      window.location.replace(
        `./paste/create_paste?paste_data=${encodeURIComponent(pasteData)}`
      );
    }
  }
}
