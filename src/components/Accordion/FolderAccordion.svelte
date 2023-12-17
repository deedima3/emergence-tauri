<script lang="ts">
  import {
    allFolderQuery,
    changeRenameFolder,
    explorerStore,
    folderQuery,
    pushHistory,
    renameFolderStore,
  } from "@/stores/explorerStore";
  import ChevronDown from "../Icons/ChevronDown.svelte";
  import FolderOpen from "../Icons/FolderOpen.svelte";
  import ChevronUp from "../Icons/ChevronUp.svelte";
  import Folder from "../Icons/Folder.svelte";
  import FolderItem from "./FolderItem.svelte";
  import { createMutationForm } from "@/hooks/createMutationForm";
  import CustomInput from "../Input/CustomInput.svelte";
  import { updateFolder } from "@/api/explorer";
  import { renameSchema } from "@/constant/schema";
  import LoadingPulse from "../Loading/LoadingPulse.svelte";

  export let folderName: string;
  export let folderID: number;
  export let onContextMenu: (e: MouseEvent, folderID: number) => void;

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

  const {
    form: { form },
    mutation: { mutation },
  } = createMutationForm({
    mutationApi: (data: { name: string }) => {
      console.log("Data", data);
      console.log("FID", folderID);
      updateFolder(folderID, data.name);
    },
    formSchema: renameSchema,
    actionName: "Update folder name",
    successFn: () => {
      changeRenameFolder(0);
      allFolderQuery.refetch();
    },
    errorFn: () => {
      changeRenameFolder(0);
      allFolderQuery.refetch();
    },
  });

  $: isSelected =
    $explorerStore.historyID[$explorerStore.selectedID] == folderID;
  $: console.log(isSelected);
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
    {#if $renameFolderStore.folderID == folderID}
      <form use:form>
        <CustomInput name="name" />
        <input type="submit" style="display: none" />
      </form>
    {:else}
      <p
        class={`text-lg text-white text-poppins ${
          isSelected ? "font-bold" : ""
        }`}
      >
        {folderName}
      </p>
    {/if}
  </div>
  {#if isSelected && folderQuery && $folderQuery.data}
    <div class="flex flex-col gap-2 ml-2">
      {#each $folderQuery.data?.files as file}
        <FolderItem fileID={file.id} filename={file.name} />
      {/each}
    </div>
  {/if}
</div>

<LoadingPulse isLoading={$mutation.isPending} />
