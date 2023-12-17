<script lang="ts">
</script>

<div
  class="w-full h-full min-h-screen bg-gradient-to-b from-gradient-start to-gradient-end"
>
  <div class="w-full flex flex-col relative z-10">
    <slot />
  </div>
  <img
    src="Stars.png"
    class="w-full max-h-screen absolute top-0 z-0"
    alt="Star Background"
  />
</div>
