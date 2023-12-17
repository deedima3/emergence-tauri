<script lang="ts">
  import FileTreeCard from "@/components/Card/FileTreeCard.svelte";
  import FolderCard from "@/components/Card/FolderCard.svelte";
  import HistoryCard from "@/components/Card/HistoryCard.svelte";
  import BackgroundLayout from "@/components/Layouts/BackgroundLayout.svelte";
  import { explorerStore } from "@/stores/explorerStore";
</script>

<BackgroundLayout>
  <div class="flex gap-5 w-full px-2 py-2">
    <div class="flex flex-col gap-2 w-64">
      <HistoryCard />
      <FileTreeCard />
    </div>
    <div class="flex flex-col gap-2 w-full">
      {#if $explorerStore.historyID[$explorerStore.selectedID] != 0 && $explorerStore.selectedID != -1}
        <FolderCard />
      {/if}
    </div>
  </div>
</BackgroundLayout>
