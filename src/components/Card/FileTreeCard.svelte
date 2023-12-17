<script lang="ts">
  import FolderAccordion from "../Accordion/FolderAccordion.svelte";
  import { allFolderQuery, setContextFolderID } from "@/stores/explorerStore";
  import FileTreeContextMenu from "../Popup/FileTreeContextMenu.svelte";

  let showMenu = false;
  // pos is cursor position when right click occur
  let pos = { x: 0, y: 0 };
  // menu is dimension (height and width) of context menu
  let menu = { h: 0, y: 0 };
  // browser/window dimension (height and width)
  let browser = { h: 0, y: 0 };

  const onContextMenu = (e: MouseEvent, folderID: number) => {
    rightClickContextMenu(e);
    setContextFolderID(folderID);
  };

  function rightClickContextMenu(e) {
    showMenu = true;
    browser = {
      w: window.innerWidth,
      h: window.innerHeight,
    };
    pos = {
      x: e.clientX,
      y: e.clientY,
    };
    if (browser.h - pos.y < menu.h) pos.y = pos.y - menu.h;
    if (browser.w - pos.x < menu.w) pos.x = pos.x - menu.w;
  }

  function onPageClick() {
    showMenu = false;
  }

  $: console.log("ShowMenu", showMenu);
</script>

<div class="flex flex-col w-64 bg-main px-2 py-5 h-full rounded-md gap-2">
  {#if $allFolderQuery && $allFolderQuery.data}
    {#each $allFolderQuery.data.folders as folder}
      <FolderAccordion
        folderID={folder.id}
        folderName={folder.name}
        {onContextMenu}
      />
    {/each}
  {/if}
</div>

<FileTreeContextMenu {onPageClick} {browser} {menu} {pos} {showMenu} />
