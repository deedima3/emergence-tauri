<script lang="ts">
  export let currentColor = "#ffffff";
  export let extraClass = "";
  export let strokeWidth = 1.5;
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  fill="none"
  viewBox="0 0 24 24"
  stroke-width={strokeWidth}
  stroke={currentColor}
  class={extraClass ? extraClass : "w-6 h-6"}
>
  <path
    stroke-linecap="round"
    stroke-linejoin="round"
    d="M8.25 4.5l7.5 7.5-7.5 7.5"
  />
</svg>
