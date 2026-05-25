export async function photoprism_get_raw(base:string, endpoint:string, key:string, timeout_millis:number):Promise<Response>
{
    const request:Request = new Request(base+endpoint);
    //request.headers.set("Authorization","Bearer " + props.photoprism_key);
    request.headers.set("Authorization","Bearer " + key);

    const request_init:RequestInit={
        signal:AbortSignal.timeout(timeout_millis)
    };

    const response = await fetch(request);
    return response;
}

export type GenericObject = {[key: string]: any };
export async function photoprism_get_json(base:string, endpoint:string, key:string, timeout_millis:number):Promise<( GenericObject|GenericObject[]|null)>
{
    let response = await photoprism_get_raw(base,endpoint,key,timeout_millis);
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

export async function photoprism_get_blob(base:string, endpoint:string, key:string, timeout_millis:number):Promise<(Blob|null)>
{
    let response = await photoprism_get_raw(base,endpoint,key,timeout_millis);
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

export const DEFAULT_TIMEOUT=5000;

export async function get_download_token(base:string, key:string):Promise<(string|null)>
{
    let endpoint="/session";
    let result=await photoprism_get_json(base,endpoint,key,DEFAULT_TIMEOUT);
    if(result!==null)
    {
        return (result as GenericObject).config.downloadToken;
    }
    else
    {
        return null;
    }
}