import { getFolderFileByFolderID, getMetaByFileID } from "@/api/explorer"
import { QueryObserver, useQueryClient } from "@tanstack/svelte-query"
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
}

const fileMetaQueryOptions = {
    queryKey : ["file-meta", get(fileMetaStore).selectedFileID],
    queryFn : () => {
        return getMetaByFileID(
            get(fileMetaStore).selectedFileID
        )
      },
}

const client = useQueryClient();
const query = new QueryObserver(client, queryOption);
const fileMetaQuery = new QueryObserver(client, fileMetaQueryOptions)

const pushHistory = (newID : string | number) => {
    let tempArray = [...get(explorerStore).historyID, newID]
    explorerStore.set({
        historyID : tempArray,
        selectedID : get(explorerStore).selectedID + 1
    })
}

const onBack = () => {
    explorerStore.set({
        historyID : get(explorerStore).historyID,
        selectedID : get(explorerStore).selectedID - 1
    })
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

explorerStore.subscribe(() => {
    query.setOptions(queryOption)
    client.invalidateQueries({
        queryKey : ["file-folder"]
    });
})

fileMetaQuery.subscribe(() => {
    query.setOptions(queryOption)
    client.invalidateQueries({
        queryKey : ["file-meta"]
    });
})


export {
    explorerStore,
    pushHistory,
    onBack,
    onForward
}