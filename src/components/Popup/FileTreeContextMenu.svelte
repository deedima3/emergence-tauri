<script lang="ts">
  import Portal from "svelte-portal";
  import DocumentUpload from "../Icons/DocumentUpload.svelte";
  import FolderOpen from "../Icons/FolderOpen.svelte";
  import { open } from "@tauri-apps/api/dialog";

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
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Image",
          extensions: ["png", "jpeg"],
        },
      ],
    });
    console.log(selected);
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
        id="navbar"
      >
        <button class="text-white flex gap-2" on:click={() => handleAddFile}>
          <DocumentUpload />
          <p>Add New File</p>
        </button>
        <button class="text-white flex gap-2">
          <FolderOpen />
          <p>Rename Folder</p>
        </button>
      </div>
    </nav>
  </Portal>
{/if}
