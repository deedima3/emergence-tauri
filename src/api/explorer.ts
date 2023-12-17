import { allFolderQuery, folderQuery } from "@/stores/explorerStore";
import type { FileMetaResponse, ImageDecryptPayload, ImageEncryptPayload, ListFileMetaResponse, ListFolderResponse } from "@/types/rust.types";
import { invoke } from "@tauri-apps/api";

const uploadFile = async (encPayload: ImageEncryptPayload) => {
    try {
        await invoke('handle_encrypt_data', {payload: encPayload})
        folderQuery.refetch()
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const getAllFolder = async (): Promise<ListFolderResponse> => {
    try {
        return await invoke('handle_get_all_folder')
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const getMetaByFileID = async (id: number ): Promise<FileMetaResponse> => {
    try {
        return await invoke('handle_get_file', {payload: {id: id}})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const getFolderFileByFolderID = async (id:  number) : Promise<ListFileMetaResponse> => {
    try {
        return await invoke('handle_get_all_file', {payload: {folder_id: id}})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    } 
}

const downloadFile = async (decPayload: ImageDecryptPayload) => {
    try {
        await invoke('handle_decrypt_data', {payload: decPayload})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const createNewFolder = async (name: string) => {
    try {
        await invoke('handle_create_folder', {payload: {name: name}})
        allFolderQuery.refetch()
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const updateFolder = async (id: number, name: string) => {
    try {
        await invoke('handle_update_folder', {payload: {folder_id: id, name: name}})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const updateFile = async (id: string, name: string) => {
    try {
        await invoke('handle_update_file', {payload: {id: id, name: name}})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const deleteFolder = async (id: number, name: string) => {
    try {
        await invoke('handle_delete_folder', {payload: {folder_id: id, name: name}})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

const deleteFile = async (id: string, name: string) => {
    try {
        await invoke('handle_delete_file', {payload: {id: id, name: name}})
    } catch (e) {
        console.log('rust-err', e);
        return Promise.reject(Error(e as string))
    }
}

export {
    uploadFile,
    getAllFolder,
    getMetaByFileID,
    getFolderFileByFolderID,
    downloadFile,
    createNewFolder,
    updateFile,
    updateFolder,
    deleteFile,
    deleteFolder
}