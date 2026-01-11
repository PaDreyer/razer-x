import { invoke } from '@tauri-apps/api/core';

export async function listRazerDevices() {
    return invoke<string[]>('get_razer_devices');
}