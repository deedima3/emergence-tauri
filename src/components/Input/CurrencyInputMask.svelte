<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField } from "felte";
  import NumberInput from "svelte-input-mask/NumberInput.svelte";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;

  const { field, onChange, onBlur } = createField(name);

  const handleChange = (e) => {
    onChange(e.detail.unmaskedValue)
  };

  const reformatValue = (value : string) => {
    console.log("ReformatParams", value)
  }
</script>

<div class="flex flex-col w-full">
  {#if label}
    <Label {name} {label} />
  {/if}
  <NumberInput
    class={`border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark ${
      disabled ? "bg-gray-300" : ""
    }`}
    on:change={handleChange}
    on:blur={onBlur}
    {name}
    {placeholder}
    {disabled}
    {...field}
    reformat={reformatValue}
  />
  <Error {name} />
</div>
