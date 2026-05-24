<script lang="ts">


    export type SlideshowProps =
    {
        photoprism_key:string|undefined
    };

    let props:SlideshowProps = $props();
    
    const base="https://photos.overdesigned.org/api/v1";

    function get(endpoint:string)
    {
        const request:Request = new Request(base+endpoint);
        request.headers.set("Authorization","Bearer " + props.photoprism_key);

        fetch(
            request
        )
        .then(
            (response)=>{
                response.json().then(
                    (json)=>{
                        console.debug(json);
                    }
                );
            }
        );
    }

    get("/albums?count=5");

</script>

<div class="main">
    
</div>

<style>
    
</style>