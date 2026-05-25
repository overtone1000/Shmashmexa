<script lang="ts">
	import { get_album_by_uid, get_all_albums, type Album } from "$lib/photoprism/albums";
	import { get_download_token } from "$lib/photoprism/commons";
    import { DEVKEY } from "$lib/photoprism/devsecrets";
	import { download_photo, get_random_photo_uid_from_album, type Photo } from "$lib/photoprism/photos";
	import { onMount } from "svelte";
    import { fly } from 'svelte/transition';

    export type SlideshowProps =
    {
        photoprism_key:string|undefined
    };

    let props:SlideshowProps = $props();

    const BASE="https://photos.overdesigned.org/api/v1";
    const ALBUM_UID="atbl6hj66z1i4hxf";
   

    let images:{blob:Blob,url:string}[] = [];
    let image_pointer:number|undefined=$state(undefined);
    let last_image_pointer:number|undefined=$derived.by(
        ()=>{
            if(image_pointer===0)
            {
                return 1;
            }
            else
            {
                return 0;
            }
        }
    );

    const millis_until_next_image=30000;

    let last_timeout:number|undefined;

    async function update_image()
    {
        //const KEY=props.photoprism_key;
        const KEY=DEVKEY; //Enable for rapid development
        if(KEY!==undefined)
        {
            console.debug("Updating image");

            let album:(Album|null)=null;
            let photo:(Photo|null)=null;
            let download_token:(string|null)=null;
            let downloaded_photo:(Blob|null)=null;

            while(album===null){
                album=await get_album_by_uid(ALBUM_UID,BASE,KEY);
            }

            while(photo===null){
                photo = await get_random_photo_uid_from_album(album,BASE,KEY);
            }

            while(download_token===null){
                download_token = await get_download_token(BASE,KEY);
            }

            while(downloaded_photo===null){
                downloaded_photo=await download_photo(photo,BASE,KEY,download_token);
            }
            
            console.debug(downloaded_photo);

            const new_image = {blob:downloaded_photo,url:URL.createObjectURL(downloaded_photo)};

            if(images.length<1)
            {
                images.push(new_image);
                image_pointer=images.length-1;
            }
            else
            {
                let target:number;
                if(image_pointer===0)
                {
                    target=1;
                }
                else
                {
                    target=0;
                }
                images[target]=new_image;
                image_pointer=target;
            }

            
        }

        clearTimeout(last_timeout);
        last_timeout=setTimeout(update_image, millis_until_next_image);
    }

    //Do this on mount so first transition works
    onMount(()=>{
        update_image();
    });

    //const FADE={delay:0,duration:1500};
    const FLY_IN={x:200,duration:3000}
    const FLY_OUT={x:-200,duration:3000}
</script>

<div class="outer">
    {#if image_pointer!==undefined && images.length>=1 && image_pointer===0}
        <img in:fly={FLY_IN} out:fly={FLY_OUT} class="image" alt="Slideshow" src={images[0].url} />
    {:else if image_pointer!==undefined && images.length>=2 && image_pointer===1}
        <img in:fly={FLY_IN} out:fly={FLY_OUT} class="image" alt="Slideshow" src={images[1].url} />
    {/if}
</div>

<style>
    .outer
    {
        width:100%;
        height:100%;
        min-width:0%;
        min-height:0%;
        flex-grow: 1;
        flex-shrink: 1;
        overflow-y: hidden;
        display: grid;
        grid-template-columns: 100%;
        grid-template-rows: 100%;
        place-items: center;
    }
    .image{
        grid-area: 1 / 1;
        max-width:100%;
        max-height:100%;
        height:auto;
        width:auto;
    }
</style>