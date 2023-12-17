<script lang="ts">
  import { folderQuery } from "@/stores/explorerStore";
  import FileCard from "./FileCard.svelte";
</script>

<div class="flex w-full gap-5 flex-wrap bg-main rounded-md p-5">
  {#if $folderQuery && $folderQuery.data}
    <div class="flex gap-5 ml-2 flex-wrap">
      {#each $folderQuery.data?.files as file}
        <FileCard
          thumbnailPath={file.thumbnail}
          filename={file.name}
          fileID={file.id}
        />
      {/each}
    </div>
  {/if}
</div>
