export interface ClientRegistrationPayload {
    password: string
    secret_key: string
}

export interface AuthPayload {
    password: string
}

export interface ImageEncryptPayload {
    path: string // full path to original img
    name: string // filename (including extensions)
    folder_id: string
    thumbnail?: string // optional. if null, no custom thumbnail will be used
}

export interface ImageDecryptPayload {
    file_id: string // file_uid 
    out_path: string // output path 
}

export interface ImageDecryptResponse {
    data: any // ??   
}

export interface FolderRequest {
    name: string
}

export interface FolderResponse {
    id: number
    name: string
}

export interface ListFolderResponse {
    folders: Array<FolderResponse>
}

export interface FileMetaResponse {
    id: number
    folder_id: number
    name: string
    file_uid: string
    file_ext: string
    encrypted_at: string
    accessed_at: string
    thumbnail: string
}

export interface ListFileMetaResponse {
    files: Array<FileMetaResponse>
}

export interface FileMetaRequest {
    id: string
    folder_id: number
}