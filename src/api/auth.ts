import type { AuthPayload, ClientRegistrationPayload } from "@/types/rust.types"
import { invoke } from "@tauri-apps/api/tauri";

const login = async (authData: AuthPayload) => {
    try {
        await invoke('handle_auth', {payload: authData})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const register = async (clientData: ClientRegistrationPayload) => {
    try {
        await invoke('handle_register_client', {payload: clientData})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const isRegistered = async (): Promise<boolean> => {
    try {
        return await invoke('handle_is_registered')
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

export {
    login,
    register, 
    isRegistered
}