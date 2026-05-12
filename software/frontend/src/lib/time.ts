const millis_per_second=1000;
const millis_per_minute=60*millis_per_second;
const millis_per_hour=60*millis_per_minute;

export function format_time_remaining(now:Date, end:Date)
{
    let millis=end.getTime()-now.getTime();
    
    const hours=Math.floor(millis/millis_per_hour);
    millis=millis%millis_per_hour;

    const minutes=Math.floor(millis/millis_per_minute);
    millis=millis%millis_per_minute

    const seconds=Math.floor(millis/millis_per_second);
    millis=millis%millis_per_second

    return format_doubledigit(hours) + ":" + format_doubledigit(minutes) + ":" + format_doubledigit(seconds);
}

export function format_doubledigit(num:number)
{
    let str=num.toString();
    if(str.length===2)
    {
        return str;
    }
    else
    {
        return "0"+str;
    }
}