<script lang="ts">
  import { BRAND_RED } from "@/constant/constant";
  import OutlineButton from "./OutlineButton.svelte";
  import Download from "../Icons/Download.svelte";

  export let onClick: () => void;
</script>

<OutlineButton {onClick}>
  <div class="flex gap-2 w-full h-full items-center justify-center">
    <div class="w-full text-base font-roboto">Export</div>
    <div class="w-5 h-5">
      <Download currentColor={BRAND_RED} />
    </div>
  </div>
</OutlineButton>
