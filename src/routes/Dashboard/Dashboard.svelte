<script lang="ts">
  import FileTreeCard from "@/components/Card/FileTreeCard.svelte";
  import HistoryCard from "@/components/Card/HistoryCard.svelte";
  import BackgroundLayout from "@/components/Layouts/BackgroundLayout.svelte";
</script>

<BackgroundLayout>
  <div class="flex gap-5 w-full px-2 py-2">
    <div class="flex flex-col gap-2 w-1/4">
      <HistoryCard />
      <FileTreeCard />
    </div>
    <div class="flex flex-col gap-2 w-full"></div>
  </div>
</BackgroundLayout>
