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
}

export interface ImageDecryptResponse {
    data: any // ??   
}