import { photoprism_get_json } from "./commons";

export type Album = {
    UID:string,
    PhotoCount:number
};

export async function get_all_albums(base:string, key:string)
{
    const max_count=5;
    let offset=0;

    let albums=[];
    let cont=true;

    while(cont)
    {
        let endpoint="/albums?count="+max_count.toString()+"&offset="+offset.toString()+"&q=type:album";
        let result=await photoprism_get_json(base,endpoint,key);
        console.debug(result);
        for(const album of result)
        {
            albums.push(album);
        }

        if(result.length>=max_count)
        {
            offset+=max_count;
        }
        else
        {
            cont=false;
        }
    }

    return albums;
}

export async function get_album_by_uid(uid:string, base:string, key:string)
{
    //This returns PhotoCount. The other query but UID does not.
    let endpoint="/albums?count=1&q=uid:"+uid;
    let result=await photoprism_get_json(base,endpoint,key);
    return result[0];
}