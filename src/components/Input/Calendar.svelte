<script lang="ts">
  import dayjs from "dayjs";

  export let value = iso(new Date());
  export let days = "Su|Mo|Tu|We|Th|Fr|Sa".split("|");
  export let months = "Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec".split(
    "|"
  );
  export let start = 0; // first day of the week (0 = Sunday, 1 = Monday)
  export let offset = 0; // offset in months from currently selected date
  export let mouseHover = false;
  export let showedValue = parseToLocale(iso(new Date()));
  export let onChange;
  export let isFlip: boolean;

  $: console.log(offset);

  function parseToLocale(date) {
    return dayjs(date).format("DD-MM-YYYY");
  }

  function iso(date) {
    const pad = (n) => (n < 10 ? "0" + n : n);
    return (
      date.getFullYear() +
      "-" +
      pad(date.getMonth() + 1) +
      "-" +
      pad(date.getDate())
    );
  }

  let date = iso(new Date());

  $: acceptDate(value);

  function acceptDate(value) {
    const newDate = new Date(value);

    if (!isNaN(newDate)) {
      date = iso(newDate);
    }
  }

  function go(direction) {
    offset = offset + direction;
  }

  function selectDate(newValue) {
    date = newValue;
    value = newValue;
    showedValue = parseToLocale(newValue);
    onChange(newValue);
    offset = 0;
  }

  $: viewDate = viewDateFrom(date, offset);

  $: month = months[viewDate.getMonth()];

  $: year = viewDate.getFullYear();

  $: weeks = weeksFrom(viewDate, date, start);

  function viewDateFrom(date, offset) {
    var viewDate = new Date(date);
    viewDate.setMonth(viewDate.getMonth() + offset);
    return viewDate;
  }

  function weeksFrom(viewDate, date, start) {
    var first = new Date(viewDate.getTime());
    first.setDate(1);
    first.setDate(first.getDate() + ((start - first.getDay() - 7) % 7));

    var last = new Date(viewDate.getTime());
    last.setDate(
      new Date(viewDate.getFullYear(), viewDate.getMonth() + 1, 0).getDate()
    );
    last.setDate(last.getDate() + ((start - last.getDay() + 6) % 7));

    var d = new Date(first.getTime()),
      M = viewDate.getMonth(),
      Y = viewDate.getFullYear(),
      week = [],
      weeks = [week];

    while (d.getTime() <= last.getTime()) {
      var dd = d.getDate(),
        mm = d.getMonth(),
        yy = d.getFullYear(),
        value = iso(d);

      week.push({
        date: dd,
        month: mm,
        value,
        class: [
          date === value ? "selected" : "",
          mm == M ? "" : (mm > M ? yy >= Y : yy > Y) ? "future" : "past",
        ].join(" "),
      });

      d = new Date(d.getFullYear(), d.getMonth(), d.getDate() + 1);

      if (d.getDay() === start) {
        week = [];
        weeks.push(week);
      }
    }

    return weeks;
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class={`bg-white px-8 py-3 shadow-md absolute z-[30] flex flex-col items-center w-96 rounded-md ${
    isFlip ? "bottom-12" : "top-12"
  }`}
  on:mouseenter={() => {
    mouseHover = true;
  }}
  on:mouseleave={() => {
    mouseHover = false;
  }}
>
  <div class="flex items-center gap-10 w-full justify-center">
    <div class="flex flex-col items-center w-full gap-3">
      <div class="flex gap-5">
        <button
          type="button"
          class="text-xl font-bold cursor-pointer"
          on:click={() => go(-12)}
        >
          {"<"}
        </button>
        <p class="text-xl font-bold">{year}</p>
        <button
          type="button"
          class="text-xl font-bold cursor-pointer"
          on:click={() => go(+12)}
        >
          {">"}
        </button>
      </div>
      <div class="flex gap-5">
        <button
          type="button"
          class="text-sm font-bold cursor-pointer"
          on:click={() => go(-1)}
        >
          {"<"}
        </button>
        <p class="text-base">{month}</p>
        <button
          type="button"
          class="text-sm font-bold cursor-pointer"
          on:click={() => go(+1)}
        >
          {">"}
        </button>
      </div>
    </div>
  </div>
  <table class="w-full mt-2">
    <tr>
      {#each days as day}
        <th>{day}</th>
      {/each}
    </tr>
    {#each weeks as week}
      <tr>
        {#each week as day}
          <td
            class={`cursor-pointer w-12 h-12 text-center ${
              dayjs(value).format("DD") == day.date &&
              parseInt(dayjs(value).format("MM")) - 1 == day.month
                ? "bg-brand-red-main rounded-xl text-white"
                : ""
            }`}
            on:click={() => selectDate(day.value)}>{day.date}</td
          >
        {/each}
      </tr>
    {/each}
  </table>
</div>
