<script lang="ts">
  import { changeSelectedFile } from "@/stores/explorerStore";
  import { convertFileSrc } from "@tauri-apps/api/tauri";

  export let thumbnailPath: string;
  export let filename: string;
  export let fileID: number;
</script>

<button
  class="flex flex-col items-center gap-2"
  on:click={() => changeSelectedFile(fileID)}
>
  <div class="w-40 h-40">
    <img
      src={convertFileSrc(thumbnailPath)}
      alt=""
      class="w-full h-full object-cover"
    />
  </div>
  <p class="text-white text-xs w-36 break-all">{filename}</p>
</button>
