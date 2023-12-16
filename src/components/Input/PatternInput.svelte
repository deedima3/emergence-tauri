<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField } from "felte";
  import { imask } from "@imask/svelte";
  import { onMount } from "svelte";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let pattern = "";
  export let disabled = false;
  export let displayName : string;

  const { field, onChange } = createField(name);

  const options = {
    mask: pattern,
    lazy: false,
  };

  let value = ''
  let showedValue = ''

  const accept = ({ detail: maskRef }) => {
    console.log('accept', maskRef.value);
    value = maskRef.unmaskedValue
    onChange(maskRef.unmaskedValue)
  }

  const complete = ({ detail: maskRef }) => {
    console.log('complete', maskRef.unmaskedValue);
  }

  onMount(() => {
    showedValue = value;
  })

</script>

<div class="flex flex-col">
  {#if label}
    <Label {name} {label} />
  {/if}
  <input
    class="border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark"
    on:accept={accept}
    on:complete={complete}
    use:imask={options}
    name={displayName}
    bind:value={showedValue}
    {placeholder}
    {disabled}
  />
  <input type="hidden" bind:value name={name} use:field>
  <Error {name} />
</div>
