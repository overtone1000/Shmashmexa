<script lang="ts">
    import { mdiCctv } from '@mdi/js';
	import { mdiCog } from '@mdi/js';
    import { mdiRefresh } from '@mdi/js';
    import IconTab from './icon_tab.svelte';
</script>

<div class="main">
    <div class="tab-row">
        <IconTab icon_label="cameras" icon_path={mdiCctv}/>
        <div class="spacer"></div>
        <IconTab icon_label="refresh" icon_path={mdiRefresh}/>
    </div>
    <iframe class="full-width" src="http://10.10.10.10:8123/dashboard-cameras/0" title="Dashboard-Cameras">
        <p>iframe unsupported</p>
    </iframe>
</div>

<style>
    .full-width
    {
        width:100%;
        flex-grow: 1;
    }
    .main
    {
        width: 100vw;
        height:100vh;
        margin: 0px;
        display:flex;
        flex-direction: column;
    }
    .tab-row
    {
        width: 100%;
        height: 50px;
        display:flex;
        flex-direction: row;
    }
    .spacer
    {
        flex-shrink: true;
        width:100%;
    }
</style>