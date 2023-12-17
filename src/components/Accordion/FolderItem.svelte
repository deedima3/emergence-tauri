<script lang="ts">
  import { changeSelectedFile } from "@/stores/explorerStore";
  import Document from "../Icons/Document.svelte";

  export let fileID: number;
  export let filename: string;
</script>

<button class="flex gap-3" on:click={() => changeSelectedFile(fileID)}>
  <Document />
  <p class="text-lg">{filename}</p>
</button>
