<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField } from "felte";
  import type { FormEventHandler } from "svelte/elements";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;

  const { field, onInput, onBlur } = createField(name);

  const handleInput : FormEventHandler<HTMLTextAreaElement> = (e) => {
    onInput(e.currentTarget.innerText);
  }
</script>

<div class="flex flex-col w-full">
  {#if label}   
    <Label {name} {label} />
  {/if}
  <textarea
    class={`border-[1px] border-gray-400 w-full h-[100px] rounded-md px-3 py-5 focus:border-brand-dark ${
      disabled ? "bg-gray-300" : ""
    }`}
    on:input={handleInput}
    on:blur={onBlur}
    {name}
    {placeholder}
    {...field}
    {disabled}
  />
  <Error {name} />
</div>
