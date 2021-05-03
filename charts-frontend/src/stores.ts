import { writable } from "svelte/store";

export const temperature = writable({ isVisible: true, data: [] });
export const humidity = writable({ isVisible: true, data: [] });
export const light = writable({ isVisible: true, data: [] });
export const soilMoisture = writable({ isVisible: true, data: [] });

export const relativeScale = writable(true);
