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

import i2pd from "./i2pd"

import * as taurifs from '@tauri-apps/plugin-fs';
import * as path from '@tauri-apps/api/path';


window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});


window.addEventListener("DOMContentLoaded", async () => {
  let inp = document.querySelector("#start-i2p-input");
  let i2p = await i2pd.getI2pd()

  let btn: HTMLButtonElement = document.querySelector("#start-i2p-form")!;
  btn.addEventListener("submit", async function (e) {
    e.preventDefault();
    await i2p.restart();
  });
});