import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
    // console.log("starting i2pd");
    // await invoke("starti2pd", {
    //   args: "--version"
    // })
  }
}


window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});


/// i2pd part
async function i2pd_init(args: string) {
  return await invoke("i2pd_init", {
    args: args
  })
}

async function i2pd_start() {
  return await invoke("i2pd_start")
}

async function i2pd_stop() {
  return await invoke("i2pd_stop")
}

async function i2pd_terminate() {
  return await invoke("i2pd_terminate")
}

let i2pd_initialised = false

window.addEventListener("DOMContentLoaded", () => {
  let inp = document.querySelector("#start-i2p-input");

  document.querySelector("#start-i2p-form")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    let args = (inp! as HTMLInputElement).value
    await i2pd_stop()
    await i2pd_terminate()
    if (!i2pd_initialised) {
      await i2pd_init(args)
      i2pd_initialised = true
    }
    await i2pd_start()
  });
});