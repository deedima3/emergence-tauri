<script lang="ts">
  import { changeSelectedFile } from "@/stores/explorerStore";
  import Document from "../Icons/Document.svelte";

  export let fileID: number;
  export let filename: string;

  const cutFilename = (filename: string) => {
    return `${filename.length > 28 ? "..." : ""}${filename.slice(-28)}`
  };
</script>

<button class="flex gap-3" on:click={() => changeSelectedFile(fileID)}>
  <Document />
  <p class="text-xs    text-white">{cutFilename(filename)}</p>
</button>
