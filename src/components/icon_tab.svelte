<script lang="ts">
	import IconButton from "./icon_button.svelte";

    export type TabProps = {
        action:()=>void,
        icon_path:string,
        icon_label:string
    };

    type TabPropsCont = {props:TabProps};
    let propscont:TabPropsCont = $props();
</script>

<div class="tab">
<IconButton
    path={propscont.props.icon_path}
    label={propscont.props.icon_label}
    action={propscont.props.action}
/>
</div>

<style>
    .tab
    {
        margin-right: 1px
    }
</style>