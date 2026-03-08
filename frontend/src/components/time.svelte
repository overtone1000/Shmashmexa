<script lang="ts">
	import { onDestroy, onMount } from "svelte";
    import "@fontsource/inter";
	import { format_doubledigit } from "$lib/time";
    
    let time=$state(new Date());

    let update_id:number|undefined=undefined;
    function update()
    {
        console.debug("Update.");
        time=new Date();
        update_id=setTimeout(update,(60-time.getSeconds())*1000);
    }

    onMount(()=>{
        update();
    });

    onDestroy(()=>{
        clearTimeout(update_id);
    });

    const months=[
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];
</script>

<div class="main">
    <div class="second time">{time.getHours()+":"+format_doubledigit(time.getMinutes())}</div>
    <div class="second date">{months[time.getMonth()] + " " + time.getDate() + ", " + time.getFullYear()}</div>
</div>

<style>
    .main
    {
        white-space: nowrap;
        font-family: Inter;
        font-size: xx-large;
        display:flex;
        flex-direction:row;
        justify-content: space-between;
        align-items: center;
        min-width: 400px;
        width: 400px;
        max-width: 400px;
    }
    .second
    {
        text-align: center;
    }
    .time
    {
        width: 28%;
    }
    .date
    {
        width:72%
    }
</style>