export async function photoprism_get_raw(base:string, endpoint:string, key:string)
{
    const request:Request = new Request(base+endpoint);
    //request.headers.set("Authorization","Bearer " + props.photoprism_key);
    request.headers.set("Authorization","Bearer " + key);

    const response = await fetch(request);
    return response;
}

export async function photoprism_get_json(base:string, endpoint:string, key:string)
{
    let response = await photoprism_get_raw(base,endpoint,key);
    const result = await response.json();
    return result;
}

export async function photoprism_get_blob(base:string, endpoint:string, key:string)
{
    let response = await photoprism_get_raw(base,endpoint,key);
    const result = await response.blob();
    return result;
}

export async function get_download_token(base:string, key:string)
{
    let endpoint="/session";
    let result=await photoprism_get_json(base,endpoint,key);
    return result.config.downloadToken;
}