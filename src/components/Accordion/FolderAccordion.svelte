<script lang="ts">
  import {
    explorerStore,
    folderQuery,
    pushHistory,
  } from "@/stores/explorerStore";
  import ChevronDown from "../Icons/ChevronDown.svelte";
  import FolderOpen from "../Icons/FolderOpen.svelte";
  import ChevronUp from "../Icons/ChevronUp.svelte";
  import Folder from "../Icons/Folder.svelte";
  import FolderItem from "./FolderItem.svelte";

  export let folderName: string;
  export let folderID: string | number;
  export let onContextMenu: (e: MouseEvent, folderID: string | number) => void;

  let draggedTo = false;

  const handleFileUpload = async (e: DragEvent) => {
    e.preventDefault();
    e.stopPropagation();

    if (!e.dataTransfer) {
      return;
    }

    const { files } = e.dataTransfer;
    draggedTo = false;
  };

  const handleDragOver = (e: {
    preventDefault: () => void;
    stopPropagation: () => void;
  }) => {
    e.preventDefault();
    e.stopPropagation();
    draggedTo = true;
  };

  const handleDragEnter = (e: {
    preventDefault: () => void;
    stopPropagation: () => void;
  }) => {
    e.preventDefault();
    e.stopPropagation();
    draggedTo = true;
  };

  const handleDragLeave = (e: {
    preventDefault: () => void;
    stopPropagation: () => void;
  }) => {
    e.preventDefault();
    e.stopPropagation();
    draggedTo = false;
  };

  $: isSelected =
    $explorerStore.historyID[$explorerStore.selectedID] == folderID;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class={`flex flex-col gap-5 ${draggedTo ? "bg-slate-700" : ""}`}
  on:drop={handleFileUpload}
  on:dragover={handleDragOver}
  on:dragenter={handleDragEnter}
  on:dragleave={handleDragLeave}
  on:contextmenu|preventDefault={(e) => onContextMenu(e, folderID)}
>
  <div class="flex gap-3 items-center">
    <button class="flex gap-1" on:click={() => pushHistory(folderID)}>
      {#if isSelected}
        <ChevronUp />
        <FolderOpen />
      {:else}
        <ChevronDown />
        <Folder />
      {/if}
    </button>
    <p
      class={`text-lg text-white text-poppins ${isSelected ? "font-bold" : ""}`}
    >
      {folderName}
    </p>
  </div>
  {#if isSelected && folderQuery}
    <div class="flex gap-2 ml-2">
      {#each $folderQuery.data as file}
        <FolderItem fileID={file.id} filename={file.filename} />
      {/each}
    </div>
  {/if}
</div>
