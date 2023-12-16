import type { ImageDecryptPayload, ImageDecryptResponse, ImageEncryptPayload } from "@/types/rust.types"
import { invoke } from "@tauri-apps/api/tauri";

const encrypt_img = async (encPayload: ImageEncryptPayload) => {
    try {
        await invoke('handle_encrypt_data', {payload: encPayload})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const decrypt_img = async (decPayload: ImageDecryptPayload): Promise<ImageDecryptResponse> => {
    try {
        return await invoke('handle_decrypt_data', {payload: decPayload})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

export {
    encrypt_img,
    decrypt_img
}