<script lang="ts">
	import { get_album_by_uid, get_all_albums } from "$lib/photoprism/albums";
	import { get_download_token } from "$lib/photoprism/commons";
    //import { DEVKEY } from "$lib/photoprism/devsecrets";
	import { download_photo, get_random_photo_uid_from_album } from "$lib/photoprism/photos";

    export type SlideshowProps =
    {
        photoprism_key:string|undefined
    };

    let props:SlideshowProps = $props();

    const BASE="https://photos.overdesigned.org/api/v1";
    const ALBUM_UID="atbl6hj66z1i4hxf";

    //get("/albums?count=5");
    
    /*
    console.debug("Getting albums.");
    get_all_albums(base,DEVKEY).then(
        (result)=>{
            console.debug(result);
        }
    );
    */

    let image:string|null = $state(null);

    const millis_until_next_image=30000;

    function update_image(key:string)
    {
        get_album_by_uid(ALBUM_UID,BASE,key).then(
            (album)=>{
                get_random_photo_uid_from_album(album,BASE,key).then(
                    (photo)=>{
                        get_download_token(BASE,key).then(
                            (download_token)=>{
                                download_photo(photo,BASE,key,download_token).then(
                                    (downloaded_photo)=>{
                                        console.debug(downloaded_photo);
                                        image=URL.createObjectURL(downloaded_photo);
                                    }
                                )
                            }
                        )
                    }
                )
            }
        );
        setTimeout(()=>{update_image(key)},millis_until_next_image);
    }

    $effect(
        ()=>
        {
            if(props.photoprism_key!==undefined)
            {
                update_image(props.photoprism_key);
            }
        }
    );
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