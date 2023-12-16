<script lang="ts">
  import { push } from "svelte-spa-router";
  import Logout from "../Icons/Logout.svelte";
  import { resetAuthUser } from "@/stores/auth";

  const handleClick = () => {
    resetAuthUser();
    push("/");
  };
</script>

<div class="w-full flex flex-col">
  <button class="w-full flex gap-5 cursor-pointer" on:click={handleClick}>
    <Logout />
    <p class={`text-white`}>Logout</p>
  </button>
</div>
