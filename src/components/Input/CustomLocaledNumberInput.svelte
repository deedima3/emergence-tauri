<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField, getValue } from "felte";
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

  let showedValue = "";
  let hiddenValue = "";

  function localeParseFloat(s: string, locale: string) {
    if (!s) {
      return 0;
    }
    let [, thousandsSeparator, , , , decimalSeparator] =
      (1111.1).toLocaleString(locale);
    s = Array.from(s, (c) =>
      c === thousandsSeparator ? "" : c === decimalSeparator ? "." : c
    ).join("");
    return parseFloat(s);
  }

  const convertToID = (value: string) => {
    console.log(
      "ConvertID",
      localeParseFloat(value, "en-US").toLocaleString("id-ID")
    );
    return localeParseFloat(value, "en-US").toLocaleString("id-ID");
  };

  const convertToUS = (value: string) => {
    console.log(
      "ConvertUS",
      localeParseFloat(value, "id-ID").toLocaleString("en-US")
    );
    return localeParseFloat(value, "id-ID").toString();
  };

  const showedOnChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    hiddenValue = convertToUS(e.currentTarget.value);
    showedValue = convertToID(hiddenValue);
    console.log("OnChange Hidden", hiddenValue);
    console.log("OnChange Showed", showedValue);
  };

  const onBlurShow = (e: FocusEvent) => {
    onChange(convertToUS(showedValue));
    hiddenValue = convertToUS(showedValue);
    console.log("OnBlurShow", convertToUS(showedValue));
  };

  data.subscribe((value) => {
    console.log("SubscribedHidden", getValue(value, name));
    if (getValue(value, name)) {
      hiddenValue = convertToUS(getValue(value, name).toString()) as string;
      showedValue = convertToID(getValue(value, name).toString() as string);
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
