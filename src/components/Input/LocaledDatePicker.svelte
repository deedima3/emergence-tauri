<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import { createField, getValue } from "felte";
  import dayjs from "dayjs";
  import PatternInput from "./PatternInput.svelte";
  import Calendar from "./Calendar.svelte";
  import { IMask, imask } from "@imask/svelte";
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";
  import type { ChangeEventHandler } from "svelte/elements";

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
    mask: "00-00-0000",
    lazy: true,
  };

  const pipe = IMask.createPipe(options);

  const value = writable(parseToISO(""));
  const showedValue = writable(parseToLocale(""));

  let focus: boolean = false;
  let mouseHover: boolean = false;

  let input: HTMLInputElement | undefined;
  $: viewportOffset = input?.getBoundingClientRect();
  $: top = viewportOffset?.top || 0;

  $: isFlip = top > 500;

  $: popupOpen = focus || mouseHover;

  function parseToLocale(date: string) {
    console.log("UnparsedDateLocale", date);
    if (date) {
      return dayjs(date).format("DD-MM-YYYY");
    } else {
      return dayjs().format("DD-MM-YYYY");
    }
  }

  function parseToISO(date: string) {
    console.log("UnparsedDateISO", date);
    if (date) {
      return dayjs(date, "DD-MM-YYYY").format("YYYY-MM-DD");
    } else {
      return dayjs().format("YYYY-MM-DD");
    }
  }

  const onChangePattern: ChangeEventHandler<HTMLInputElement> = (e) => {
    value.set(e.currentTarget.value);
    showedValue.set(pipe(e.currentTarget.value));
    onChange(e.currentTarget.value);
    console.log(value);
    console.log(showedValue);
  };

  data.subscribe((data) => {
    if (data[name]) {
      value.set(parseToISO(getValue(data, name) as string));
      showedValue.set(parseToLocale(getValue(data, name) as string));
    }
    return;
  });
</script>

<div class="flex flex-col w-full">
  {#if label}
    <Label {name} {label} />
  {/if}
  <div class="w-full relative">
    <input
      class={`border-[1px] border-gray-400 w-full h-[30px] rounded-md px-3 py-5 focus:border-brand-dark ${
        disabled ? "bg-gray-300" : ""
      }`}
      bind:value={$showedValue}
      bind:this={input}
      on:change={onChangePattern}
      on:focus={() => {
        focus = true;
      }}
      on:blur={() => {
        focus = false;
      }}
      {placeholder}
      {disabled}
    />
    {#if popupOpen}
      <Calendar
        bind:mouseHover
        bind:value={$value}
        bind:showedValue={$showedValue}
        {isFlip}
        {onChange}
      />
    {/if}
  </div>
  <input type="hidden" bind:value={$value} {name} use:field />
  <Error {name} />
</div>
