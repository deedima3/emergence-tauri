import { getAllFolder, getFolderFileByFolderID, getMetaByFileID } from "@/api/explorer"
import { QueryClient, QueryObserver } from "@tanstack/svelte-query"
import toast from "svelte-french-toast"
import { get, writable, type Writable } from "svelte/store"

type ExplorerStore = {
    historyID : (string | number)[];
    selectedID : number;
}

type FileMetaStore = {
    selectedFileID : number
}

const explorerStore : Writable<ExplorerStore> = writable({
    historyID : [],
    selectedID : 0,
})

const fileMetaStore : Writable<FileMetaStore> = writable({
    selectedFileID : 0
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

const fileMetaQueryOptions = {
    queryKey : ["file-meta", get(fileMetaStore).selectedFileID],
    queryFn : () => {
        return getMetaByFileID(
            get(fileMetaStore).selectedFileID
        )
      },
    enabled : get(fileMetaStore).selectedFileID != 0
}

const allFolderOptions = {
    queryKey : ["all-folder"],
    queryFn : () => {
        return getAllFolder()
    }
}

const client = new QueryClient();
const folderQuery = new QueryObserver(client, queryOption);
const fileMetaQuery = new QueryObserver(client, fileMetaQueryOptions)
const allFolderQuery = new QueryObserver(client, allFolderOptions)

const pushHistory = (newID : string | number) => {
    let tempArray = [...get(explorerStore).historyID, newID]
    explorerStore.set({
        historyID : tempArray,
        selectedID : get(explorerStore).selectedID + 1
    })
}

const onBack = () => {
    if(get(fileMetaStore).selectedFileID != 0){
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

const changeSelectedFile = (id : number) => {
    if(get(fileMetaStore).selectedFileID){
        fileMetaStore.set({
            selectedFileID : id
        })
    } else {
        fileMetaStore.set({
            selectedFileID : 0
        })
    }
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


export {
    explorerStore,
    pushHistory,
    onBack,
    onForward,
    changeSelectedFile,
    folderQuery,
    fileMetaQuery,
    allFolderQuery
}