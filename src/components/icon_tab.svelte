<script lang="ts">
	import IconButton from "./icon_button.svelte";

    type TabProps = {
        action:()=>void,
        icon_path:string,
        icon_label:string
    };
    let { 
        action,
        icon_path,
        icon_label
    }:TabProps = $props();
</script>

<IconButton
    path={icon_path}
    label={icon_label}
/>

<style>
</style>