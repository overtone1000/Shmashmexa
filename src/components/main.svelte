<script lang="ts">
    function navigate (dest:string) {window.location.href=dest;}
</script>

<div class="main">
    Hello world
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
</style>