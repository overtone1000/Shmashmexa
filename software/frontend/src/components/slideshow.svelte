<script lang="ts">
	import { get_album_by_uid, get_all_albums, type Album } from "$lib/photoprism/albums";
	import { get_download_token } from "$lib/photoprism/commons";
    //import { DEVKEY } from "$lib/photoprism/devsecrets";
	import { download_photo, get_random_photo_uid_from_album, type Photo } from "$lib/photoprism/photos";

    export type SlideshowProps =
    {
        photoprism_key:string|undefined
    };

    let props:SlideshowProps = $props();

    const BASE="https://photos.overdesigned.org/api/v1";
    const ALBUM_UID="atbl6hj66z1i4hxf";
   

    let image:string|null = $state(null);

    const millis_until_next_image=30000;

    async function update_image()
    {
        const KEY=props.photoprism_key;
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
            image=URL.createObjectURL(downloaded_photo);   
        }

        setTimeout(update_image, millis_until_next_image);
    }

    update_image();
</script>

<div class="outer">
    <img class="image" alt="Slideshow" src={image} />
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
        display:flex;
        justify-content: center;
    }
    .image{
        max-width:100%;
        max-height:100%;
        height:auto;
        width:auto;
    }
</style>