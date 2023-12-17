<script lang="ts">
  import Portal from "svelte-portal";
  import DocumentUpload from "../Icons/DocumentUpload.svelte";
  import FolderOpen from "../Icons/FolderOpen.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { deleteFolder, uploadFile } from "@/api/explorer";
  import { first, last } from "radash";
  import {
    changeRenameFolder,
    contextFileStore,
    resetFolderHistory,
  } from "@/stores/explorerStore";
  import toast from "svelte-french-toast";
  import Edit from "../Icons/Edit.svelte";
  import Delete from "../Icons/Delete.svelte";

  // pos is cursor position when right click occur
  export let pos = { x: 0, y: 0 };
  // menu is dimension (height and width) of context menu
  export let menu = { h: 0, y: 0 };
  // browser/window dimension (height and width)
  export let browser = { h: 0, y: 0 };
  // showMenu is state of context-menu visibility
  export let showMenu: boolean;
  export let onPageClick: () => void;

  function getContextMenuDimension(node) {
    // This function will get context menu dimension
    // when navigation is shown => showMenu = true
    let height = node.offsetHeight;
    let width = node.offsetWidth;
    menu = {
      h: height,
      w: width,
    };
  }

  const handleAddFile = async () => {
    console.log("selected");
    const selected = await open({
      multiple: false,
      filters: [],
    });
    await uploadFile({
      path: selected as string,
      name: last((selected as string).split(`\\`)) as string,
      folder_id: $contextFileStore.folderID,
    });
    toast.success("File berhasil diupload!");
  };
</script>

{#if showMenu}
  <Portal>
    <nav
      use:getContextMenuDimension
      style="position: absolute; top:{pos.y}px; left:{pos.x}px"
      class="z-20"
      on:mouseleave={onPageClick}
    >
      <div
        class="bg-main w-max h-full px-5 text-white py-2 rounded-md shadow-md flex flex-col gap-3"
      >
        <button
          class="text-white flex gap-2"
          on:click={() => {
            handleAddFile();
            onPageClick();
          }}
        >
          <DocumentUpload />
          <p>Add New File</p>
        </button>
        <button
          class="text-white flex gap-2"
          on:click={() => {
            changeRenameFolder($contextFileStore.folderID);
            onPageClick();
          }}
        >
          <Edit />
          <p>Rename Folder</p>
        </button>
        <button
          class="text-white flex gap-2"
          on:click={() => {
            deleteFolder($contextFileStore.folderID);
            resetFolderHistory();
            onPageClick();
          }}
        >
          <Delete />
          <p>Delete Folder</p>
        </button>
      </div>
    </nav>
  </Portal>
{/if}
