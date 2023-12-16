<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import type { CustomOption } from "@/interfaces/ui.interfaces";
  import { createField } from "felte";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let options: CustomOption[] = [];
  export let disabled = false;

  let value = "";

  const { onInput, onBlur } = createField(name);

  $: onInput(value);
</script>

<div class="flex flex-col w-full">
  {#if label}
    <Label {name} {label} />
  {/if}
  <select
    class={`border-[1px] border-gray-400 w-full h-[42px] rounded-md px-3 focus:border-brand-dark ${
      disabled ? "bg-gray-300" : "bg-white"
    }`}
    {name}
    {placeholder}
    on:blur={onBlur}
    bind:value
    {disabled}
  >
    {#if placeholder}
      <option class="" value="" disabled>
        {placeholder}
      </option>
    {/if}
    {#each options as option}
      <option class="" value={option.value}>
        {option.label}
      </option>
    {/each}
  </select>
  <Error {name} />
</div>
