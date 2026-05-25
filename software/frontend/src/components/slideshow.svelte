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

    async function update_image(key:string|undefined)
    {
        if(key!==undefined)
        {
            console.debug("Updating image");

            let album:(Album|null)=null;
            let photo:(Photo|null)=null;
            let download_token:(string|null)=null;
            let downloaded_photo:(Blob|null)=null;

            while(album===null){
                album=await get_album_by_uid(ALBUM_UID,BASE,key);
            }

            while(photo===null){
                photo = await get_random_photo_uid_from_album(album,BASE,key);
            }

            while(download_token===null){
                download_token = await get_download_token(BASE,key);
            }

            while(downloaded_photo===null){
                downloaded_photo=await download_photo(photo,BASE,key,download_token);
            }
            
            console.debug(downloaded_photo);
            image=URL.createObjectURL(downloaded_photo);   
        }
    }

    let update=()=>{update_image(props.photoprism_key);}
    //let update=()=>{update_image(DEVKEY);}

    update();

    setInterval(update, millis_until_next_image);
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