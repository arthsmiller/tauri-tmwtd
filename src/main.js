const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let currentTime;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function update_current_time() {
  currentTime.textContent = await invoke("update_current_time", { name: currentTime.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  currentTime = document.querySelector("#current-time");

  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  setInterval(update_current_time, 1000);
});
