<script>
  import FileTreeCard from "@/components/Card/FileTreeCard.svelte";
  import HistoryCard from "@/components/Card/HistoryCard.svelte";
  import BackgroundLayout from "@/components/Layouts/BackgroundLayout.svelte";
</script>

<BackgroundLayout>
  <div class="flex gap-5 w-full">
    <div class="flex flex-col gap-2 w-1/5">
      <HistoryCard />
      <FileTreeCard />
    </div>
    <div class="flex flex-col gap-2 w-full"></div>
  </div>
</BackgroundLayout>
