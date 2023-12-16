<script lang="ts">
  export let currentColor = "#ffffff";
  export let extraClass = "";
</script>

<svg 
xmlns="http://www.w3.org/2000/svg"
  fill="none"
  viewBox="0 0 24 24"
  stroke-width="1.5"
  stroke={currentColor}
  class={extraClass ? extraClass : "w-6 h-6"}>
  <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
  <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
</svg>

