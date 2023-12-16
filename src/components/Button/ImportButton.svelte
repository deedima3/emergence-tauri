<script lang="ts">
  import { BRAND_RED } from "@/constant/constant";
  import Upload from "../Icons/Upload.svelte";
  import OutlineButton from "./OutlineButton.svelte";

  export let onClick : () => void
</script>

<OutlineButton {onClick}>
  <div class="flex gap-2 w-full h-full items-center justify-center">
    <div class="w-full text-base font-roboto">Import</div>
    <div class="w-5 h-5">
      <Upload currentColor={BRAND_RED} />
    </div>
  </div>
</OutlineButton>
