<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField, getValue } from "felte";
  import { IMask } from "@imask/svelte";
  import type { ChangeEventHandler } from "svelte/elements";
  import type { Writable } from "svelte/store";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;
  export let data: Omit<Writable<any>, "subscribe"> & {
    subscribe(
      subscriber: (values: { [x: string]: unknown }) => any
    ): () => void;
  } & Record<string, any>;

  const { field, onChange } = createField(name);

  const options = {
    mask: Number,
    lazy: true,
    thousandsSeparator: ".",
    mapToRadix: [","],
    normalizeZeros: false,
    radix: ",",
  };

  const pipe = IMask.createPipe(options);

  let showedValue = "";
  let hiddenValue = "";

  const showedOnChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    hiddenValue = e.currentTarget.value.replace(",", ".");
    showedValue = `${pipe(hiddenValue.replace(".", ","))}`;
    console.log("OnChange Hidden", hiddenValue);
    console.log("OnChange Showed", showedValue);
  };

  const onBlurShow = (e: FocusEvent) => {
    onChange(parseFloat(showedValue.replace(",", ".")));
    console.log("OnBlurShow", showedValue.replace(",", "."));
  };

  data.subscribe((value) => {
    console.log("SubscribedHidden", getValue(value, name));
    hiddenValue = getValue(value, name) as string;
    if (getValue(value, name)) {
      showedValue = `${pipe(
        getValue(value, name).toString().replace(".", ",")
      )}`;
    }
  });

  $: console.log("HiddenValue", hiddenValue);
  $: console.log("ShowedValue", showedValue);
</script>

<div class="flex flex-col w-full">
  {#if label}
    <Label {name} {label} />
  {/if}
  <input
    class={`border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark ${
      disabled ? "bg-gray-300" : ""
    }`}
    bind:value={showedValue}
    on:change={showedOnChange}
    on:blur={onBlurShow}
    {placeholder}
    {disabled}
  />
  <input type="hidden" bind:value={hiddenValue} {name} use:field />
  <Error {name} />
</div>
