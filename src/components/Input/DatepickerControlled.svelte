<script lang="ts">
  import Label from "./Label.svelte";
  import Error from "./Error.svelte";
  import dayjs from "dayjs";
  import Calendar from "./Calendar.svelte";
  import { parseDateLocaled } from "@/utils/utils";

  export let name: string;
  export let label: string = "";
  export let placeholder = "";
  export let disabled = false;
  export let value = parseToISO("");
  export let showedValue = parseToLocale("");

  $: console.log("ParseDateLocale", showedValue);

  let clicked: boolean = false;
  let mouseHover: boolean = false;

  let button: HTMLButtonElement | undefined;
  $: viewportOffset = button?.getBoundingClientRect();
  $: top = viewportOffset?.top || 0;

  $: isFlip = top > 400;

  $: popupOpen = clicked || mouseHover;

  const onChange = (date: string) => {
    showedValue = parseDateLocaled(date);
    value = date;
  };

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
</script>

<div class="flex flex-col w-36">
  {#if label}
    <Label {name} {label} />
  {/if}
  <div class="w-36 relative">
    <button
      class={`w-36 px-2 py-3  rounded-md text-white font-bold flex items-center justify-center ${
        disabled ? "bg-slate-500" : "bg-brand-light"
      }`}
      bind:this={button}
      type="button"
      on:click={() => {
        clicked = true;
      }}
      on:blur={() => {
        clicked = false;
      }}
    >
      {showedValue ? parseDateLocaled(parseToISO(showedValue)) : placeholder}
    </button>
    {#if popupOpen}
      <Calendar {isFlip} bind:value bind:showedValue {onChange} />
    {/if}
  </div>
  <input type="hidden" bind:value {name} />
  <Error {name} />
</div>
