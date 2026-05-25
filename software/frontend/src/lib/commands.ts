export type TabConfig = {
    url:string,
    priority:number,
    timeout_seconds:number
};

export type AutoTabEntry = {
    config:TabConfig,
    expiry:Date
};

export type Command = {
    AutoTab?:string,
    PhotoprismKey?:string,
    SetScreenState?:boolean
}