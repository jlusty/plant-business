import { writable } from "svelte/store";

export const temperature = writable({ isVisible: true });
export const humidity = writable({ isVisible: true });
export const light = writable({ isVisible: true });
export const soilMoisture = writable({ isVisible: true });

export const relativeScale = writable(false);
