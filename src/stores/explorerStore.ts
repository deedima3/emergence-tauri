import { getAllFolder, getFolderFileByFolderID, getMetaByFileID } from "@/api/explorer"
import { reporter } from "@felte/reporter-svelte"
import { validator } from "@felte/validator-zod"
import { QueryClient, QueryObserver } from "@tanstack/svelte-query"
import { createForm } from "felte"
import toast from "svelte-french-toast"
import { get, writable, type Writable } from "svelte/store"

type ExplorerStore = {
    historyID : number[];
    selectedID : number;
}

type FileMetaStore = {
    selectedFileUID : string
}

type ContextFolderStore = {
    folderID : number
}

const explorerStore : Writable<ExplorerStore> = writable({
    historyID : [],
    selectedID : - 1,
})

const fileMetaStore : Writable<FileMetaStore> = writable({
    selectedFileUID : "0"
})

const contextFileStore:  Writable<ContextFolderStore> = writable({
    folderID : 0
})

const renameFolderStore: Writable<ContextFolderStore> = writable({
    folderID : 0
})


const queryOption = {
    queryKey : ["file-folder", get(explorerStore).historyID[get(explorerStore).selectedID]],
    queryFn : () => {
        return getFolderFileByFolderID(
            get(explorerStore).historyID[get(explorerStore).selectedID]
        )
      },
    enabled : get(explorerStore).historyID[get(explorerStore).selectedID] != 0
}

// const fileMetaQueryOptions = {
//     queryKey : ["file-meta", get(fileMetaStore).selectedFileID],
//     queryFn : () => {
//         return getMetaByFileID(
//             get(fileMetaStore).selectedFileID
//         )
//       },
//     enabled : get(fileMetaStore).selectedFileID != 0
// }

const allFolderOptions = {
    queryKey : ["all-folder"],
    queryFn : () => {
        return getAllFolder()
    }
}

const client = new QueryClient();
const folderQuery = new QueryObserver(client, queryOption);
// const fileMetaQuery = new QueryObserver(client, fileMetaQueryOptions)
const allFolderQuery = new QueryObserver(client, allFolderOptions)

const setContextFolderID = (id : number) => {
    contextFileStore.set({
        folderID : id
    })
}

const pushHistory = (newID : number) => {
    let tempArray = [...get(explorerStore).historyID, newID]
    if(!(newID == get(explorerStore).historyID[get(explorerStore).selectedID])){
        explorerStore.set({
            historyID : tempArray,
            selectedID : get(explorerStore).selectedID + 1
        })
    } else {
        explorerStore.set({
            historyID : [...get(explorerStore).historyID, 0],
            selectedID : get(explorerStore).selectedID + 1
        })
    }
}

const onBack = () => {
    if(get(explorerStore).selectedID != 0){
        explorerStore.set({
            historyID : get(explorerStore).historyID,
            selectedID : get(explorerStore).selectedID - 1
        })
    } else {
        toast.error("Tidak bisa mundur lagi!")
    }
}

const onForward = () => {
    let historyArray = get(explorerStore).historyID
    let selectedID = get(explorerStore).selectedID
    if(historyArray.length < selectedID){
        explorerStore.set({
            historyID : get(explorerStore).historyID,
            selectedID : get(explorerStore).selectedID + 1
        })
    } else {
        toast.error("Tidak bisa maju lagi!")
    }
}

const changeSelectedFile = (id : string) => {
    if(get(fileMetaStore).selectedFileUID){
        fileMetaStore.set({
            selectedFileUID : id
        })
    } else {
        fileMetaStore.set({
            selectedFileUID : "0"
        })
    }
}

const changeRenameFolder = (id : number) => {
    renameFolderStore.set({
        folderID : id
    })
}

const resetFolderHistory = () => {
    explorerStore.set({
        historyID : [],
        selectedID : - 1,
    })
}

explorerStore.subscribe(() => {
    folderQuery.setOptions(queryOption)
    client.invalidateQueries({
        queryKey : ["file-folder"]
    });
})

fileMetaStore.subscribe(() => {
    folderQuery.setOptions(queryOption)
    client.invalidateQueries({
        queryKey : ["file-meta"]
    });
})

explorerStore.subscribe((value) => {
    console.log(value)
})

fileMetaStore.subscribe((value) => {
    console.log(value)
})

contextFileStore.subscribe((value) => {
    console.log(value)
})


export {
    explorerStore,
    pushHistory,
    onBack,
    onForward,
    changeSelectedFile,
    folderQuery,
    // fileMetaQuery,
    allFolderQuery, 
    setContextFolderID,
    contextFileStore,
    renameFolderStore,
    changeRenameFolder,
    resetFolderHistory,
    fileMetaStore
}