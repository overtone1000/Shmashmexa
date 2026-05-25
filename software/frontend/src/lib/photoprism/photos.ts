import type { Album } from "./albums";
import { DEFAULT_TIMEOUT, photoprism_get_blob, photoprism_get_json, type GenericObject } from "./commons";

export function bounded_random_integer(minimum:number,maximum:number){
    return Math.floor(Math.random()*(maximum-minimum+1)+minimum);
}

export type Photo = {
    UID:string
}

export async function get_random_photo_uid_from_album(album:Album, base:string, key:string, last_offset:number)
{
    let offset=last_offset;
    while(offset===last_offset)
    {
        offset=bounded_random_integer(0,album.PhotoCount-1);
    }
    let endpoint="/photos?count=1&offset="+offset+"&s="+album.UID;

    let result=await photoprism_get_json(base,endpoint,key,DEFAULT_TIMEOUT);

    if(result!==null)
    {
        return {last_offset:offset, photo:(result as Photo[])[0]};
    }
    else
    {
        return null;
    }
}

export async function download_photo(photo:Photo, base:string, key:string, download_token:string, timeout_millis:number)
{
    let endpoint="/photos/"+photo.UID+"/dl?t="+download_token;
    console.debug(endpoint);
    let result=await photoprism_get_blob(base,endpoint,key,timeout_millis); //allow for a long download time
    return result;
}