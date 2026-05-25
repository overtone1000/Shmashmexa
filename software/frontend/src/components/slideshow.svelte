<script lang="ts">
	import { get_album_by_uid, get_all_albums, type Album } from "$lib/photoprism/albums";
	import { get_download_token } from "$lib/photoprism/commons";
    import { DEVKEY } from "$lib/photoprism/devsecrets";
	import { download_photo, get_random_photo_uid_from_album, type Photo } from "$lib/photoprism/photos";
	import { onDestroy, onMount } from "svelte";
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
    let last_photo_offset:number=-1;

    async function update_image()
    {
        //const KEY=props.photoprism_key;
        const KEY=DEVKEY; //Enable for rapid development
        if(KEY!==undefined)
        {
            console.debug("Updating image");

            const album:(Album|null)=await get_album_by_uid(ALBUM_UID,BASE,KEY);
            if(album===null){return;}
            const photo_result=await get_random_photo_uid_from_album(album,BASE,KEY,last_photo_offset);
            if(photo_result===null){return;}
            const download_token:(string|null)=await get_download_token(BASE,KEY);
            if(download_token===null){return;}
            const downloaded_photo:(Blob|null)=await download_photo(photo_result.photo,BASE,KEY,download_token,millis_until_next_image);
            if(downloaded_photo===null){return;}

            last_photo_offset=photo_result.last_offset;
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
    }

    let interval_id:number;
    onMount(()=>{
        update_image();
        interval_id=setInterval(update_image,millis_until_next_image);
    });

    onDestroy(()=>{
        clearInterval(interval_id);
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
        overflow-x: hidden;
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