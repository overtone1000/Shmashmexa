export type TabConfig = {
    url:string,
    priority:number,
    timeout_seconds:number
};

export type Command = {
    AutoTab?:TabConfig
}