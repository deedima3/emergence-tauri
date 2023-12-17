<script lang="ts">
  import { onBack, onForward } from "@/stores/explorerStore";
  import ArrowLeft from "../Icons/ArrowLeft.svelte";
  import ArrowRight from "../Icons/ArrowRight.svelte";
  import FolderPlus from "../Icons/FolderPlus.svelte";
  import { createNewFolder } from "@/api/explorer";
</script>

<div class="flex bg-main rounded-full gap-5 py-5 px-5 justify-between">
  <button class="" on:click={onBack}>
    <ArrowLeft />
  </button>
  <button class="" on:click={onForward}>
    <ArrowRight />
  </button>
  <button class="" on:click={() => createNewFolder("New Folder")}>
    <FolderPlus />
  </button>
</div>
