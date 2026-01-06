import HoleApp from "./hole/HoleApp.svelte";
import WheelApp from "./wheel/WheelApp.svelte";

const params = new URLSearchParams(window.location.search);
const target = document.getElementById("app");
const windowType = params.get("window") ?? "hole";

if (!target) {
  throw new Error("Missing app root");
}

if (windowType === "wheel") {
  new WheelApp({ target });
} else {
  new HoleApp({ target });
}
