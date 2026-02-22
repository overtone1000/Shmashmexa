<script lang="ts">
    type Props = {
        path:string,
        label:string
    };
    let { 
        path,
        label
    }:Props = $props();
</script>

<button
    class="iconbutton"
    aria-label={label}
>
    <svg viewBox="0 0 24 24">
        <path class="iconsvg" d={path}/>
    </svg>
</button>

<style>
    .iconbutton{
        border-radius: 5%;
        width:48px;
        height:48px;
        align-self: center;
        margin:2px;
        padding:0px;
        margin: 0px;
        border-width: 2px;
    }
    .iconsvg{
        fill:none;
        stroke:black;
        stroke-width:1;
    }
</style>