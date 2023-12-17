<script lang="ts">
  import { changeSelectedFile } from "@/stores/explorerStore";
  import { convertFileSrc } from "@tauri-apps/api/tauri";

  export let thumbnailPath: string;
  export let filename: string;
  export let fileUID: string;
  export let fileID: number;
  export let onContextMenu: (e: MouseEvent, fileUID: string) => void;
</script>

<button
  class="flex flex-col items-center gap-2"
  on:click={() => changeSelectedFile(fileUID)}
  on:contextmenu|preventDefault={(e) => onContextMenu(e, fileUID)}
>
  <div class="w-16 h-16">
    <img
      src={convertFileSrc(thumbnailPath)}
      alt=""
      class="w-full h-full object-cover"
    />
  </div>
  <p class="text-white text-xs w-36 break-all">{filename}</p>
</button>
