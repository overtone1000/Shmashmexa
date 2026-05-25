export async function photoprism_get_raw(base:string, endpoint:string, key:string)
{
    const request:Request = new Request(base+endpoint);
    //request.headers.set("Authorization","Bearer " + props.photoprism_key);
    request.headers.set("Authorization","Bearer " + key);

    const response = await fetch(request);
    return response;
}

export async function photoprism_get_json(base:string, endpoint:string, key:string):Promise<(any|null)>
{
    let response = await photoprism_get_raw(base,endpoint,key);
    if(response.ok)
    {
        const result = await response.json();
        return result;
    }
    else
    {
        console.debug(response.status);
        return null;
    }
}

export async function photoprism_get_blob(base:string, endpoint:string, key:string):Promise<(Blob|null)>
{
    let response = await photoprism_get_raw(base,endpoint,key);
    if(response.ok)
    {
        const result = await response.blob();
        return result;
    }
    else
    {
        console.debug(response.status);
        return null;
    }    
}

export async function get_download_token(base:string, key:string)
{
    let endpoint="/session";
    let result=await photoprism_get_json(base,endpoint,key);
    if(result!==null)
    {
        return result.config.downloadToken;
    }
    else
    {
        return null;
    }
}