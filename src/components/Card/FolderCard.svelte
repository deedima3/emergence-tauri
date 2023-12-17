<script lang="ts">
  import { changeSelectedFile, folderQuery } from "@/stores/explorerStore";
  import FileCard from "./FileCard.svelte";
  import FileCardContextMenu from "../Popup/FileCardContextMenu.svelte";

  let showMenu = false;
  // pos is cursor position when right click occur
  let pos = { x: 0, y: 0 };
  // menu is dimension (height and width) of context menu
  let menu = { h: 0, y: 0 };
  // browser/window dimension (height and width)
  let browser = { h: 0, y: 0 };

  const onContextMenu = (e: MouseEvent, fileUID: string) => {
    rightClickContextMenu(e);
    changeSelectedFile(fileUID);
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

<div class="flex w-full gap-5 flex-wrap bg-main rounded-md p-5">
  {#if $folderQuery && $folderQuery.data}
    <div class="flex gap-5 ml-2 flex-wrap">
      {#each $folderQuery.data?.files as file}
        <FileCard
          {onContextMenu}
          thumbnailPath={file.thumbnail}
          filename={file.name}
          fileID={file.id}
          fileUID={file.file_uid}
        />
      {/each}
    </div>
  {/if}
</div>

<FileCardContextMenu {onPageClick} {browser} {menu} {pos} {showMenu} />
