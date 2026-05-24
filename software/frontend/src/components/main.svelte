<script lang="ts">
    import { mdiClock } from '@mdi/js';
    import { mdiRefresh } from '@mdi/js';
    import { mdiRobot } from '@mdi/js';
    import IconTab, { type TabProps } from './icon_tab.svelte';
	import { onMount } from 'svelte';
	import Time from './time.svelte';
	import TimerPage, { type Timer, type TimerState as TimerState } from './timer_page.svelte';
	import type { AutoTabEntry, Command, TabConfig } from '$lib/commands';

    console.debug("Starting main.");

    enum MainField {
        iframe,
        component
    };

    type IFrameMeta = {
        url:string|null,
        title:string
    };

    enum ComponentType {
        clock
    };

    type Main = {
        field:MainField,
        iframe_meta?:IFrameMeta
        component_meta?:ComponentType
    };


    type TabsConfig = {
        label:string,
        title:string,
        icon_path:string,
        url:string
    };

    let main:Main|undefined = $state(undefined);

    const timers:Timer[] = $state([]);

    let tabs:TabProps[]|undefined = $state(undefined);
    
    const refresh:TabProps = {
        action: () => {
            location.reload();
        },
        icon_label: "refresh",
        icon_path: mdiRefresh
    };

    const clock:TabProps = {
        action: () => {
            main={
                field: MainField.component,
                component_meta:ComponentType.clock
            }
        },
        icon_label: "clock",
        icon_path: mdiClock,
        disabled: true //Not ready yet
    };

    function set_manual_tab(tab_props:TabProps)
    {
        manual_tab_props=tab_props;
        active_tab_props=tab_props;
        //tab_props.action(); //Don't need to do this, it will run in the effect below.
    }

    let manual_tab_props=$state<TabProps|null>(null);
    let auto_tab_config=$state<TabConfig|null>(null);

    let active_tab_props=$state<TabProps|null>(null);

    $effect(()=>{
        active_tab_props?.action();
    });
        
    let auto_tabs:Set<AutoTabEntry>=new Set();

    let auto_tab_props = $derived(
        {
            action: () => {
                if(auto_tab_config!==null)
                {
                    main={
                        field: MainField.iframe,
                        iframe_meta:{
                            url: auto_tab_config.url,
                            title: "Automatic Tab"
                        }
                    }
                }
            },
            icon_label: "auto",
            icon_path: mdiRobot,
            disabled: auto_tab_config===null
        }
    );

    function update_auto_tab_state(update_active_tab:boolean)
    {
        console.debug("Updating auto tab state.",update_active_tab);

        let selected_config:undefined|AutoTabEntry=undefined;
        const now=new Date();
        for(const auto_tab_entry of auto_tabs)
        {
            if(auto_tab_entry.expiry<now)
            {
                auto_tabs.delete(auto_tab_entry);
            }
            else
            {
                if(selected_config===undefined || 
                    auto_tab_entry.config.priority>selected_config.config.priority ||
                    (
                        auto_tab_entry.config.priority==selected_config.config.priority &&
                        auto_tab_entry.expiry<selected_config.expiry
                    )
                )
                {
                    selected_config=auto_tab_entry;
                }
            }
        }

        console.debug("Selected state is",selected_config);

        if(selected_config!==undefined)
        {
            if(selected_config.config!==auto_tab_config)
            {
                auto_tab_config=selected_config.config;   
            }
            if(update_active_tab)
            {
                active_tab_props=auto_tab_props;
            }
            let wait=(selected_config.expiry.getTime()-now.getTime());
            console.debug("Setting timeout for update.",wait);
            setTimeout(()=>{update_auto_tab_state(false)},wait);
        }
        else
        {
            auto_tab_config=null;
            if(active_tab_props!==manual_tab_props)
            {
                active_tab_props=manual_tab_props;
            }
        }
    }

    function handle_server_command(command:Command)
    {
        console.debug("Handling command.");
        if(command.AutoTab)
        {
            let auto_tab:TabConfig=JSON.parse(command.AutoTab);
            console.debug("Received auto tab.",auto_tab);
            let expiry:Date = new Date(Date.now()+auto_tab.timeout_seconds*1000);

            const entry:AutoTabEntry = {
                config:auto_tab,
                expiry:expiry
            };

            auto_tabs.add(entry);
            update_auto_tab_state(true);
        }
    }

    function open_socket(){
        const socket_url = "ws:/"+location.host;
        console.debug("Opening websocket on");
        const socket = new WebSocket(socket_url);

        //const test = () => {
        //    console.debug("Test...");
        //    socket.send(new Date().toString());
        //    setTimeout(test,1000);
        //}

        // Connection opened
        socket.addEventListener("open", (event) => {
            console.debug("Connection opened.");
            //test();
        });

        // Listen for messages
        socket.addEventListener("message", (event) => {
            console.log("Message from server ", event.data);
            handle_server_command(JSON.parse(event.data));
        });
    };

    function build_tabs(tabs_config:TabsConfig[]) {
        if(tabs_config.length>0)
        {
            tabs = [];

            for(const tabconfig of tabs_config)
            {
                tabs.push(
                    {
                        icon_label: tabconfig.label,
                        icon_path: tabconfig.icon_path,
                        action: ()=>{
                            main={
                                field: MainField.iframe,
                                iframe_meta:{
                                    url: tabconfig.url,
                                    title: tabconfig.title
                                }
                            }
                        }
                    }
                );
            }

            //Default to zero
            set_manual_tab(tabs[0]);
        }
    }

    async function get_tabs() {
        const url = location.origin+"/config/tabs.json";
        console.debug("Getting tabs from " + url);
        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }
            else
            {
                const result = await response.json();
                build_tabs(result);
            }
        } catch (error:any) {
            console.error(error.message);
        }
    }

    onMount(()=>{
        open_socket();
        get_tabs();
    });

</script>

<div class="main">
    <div class="tab-row">
        {#each tabs as tab}
            <IconTab --right_margin="4px" props={tab}/>
        {/each}
        <IconTab props={clock}/>
        <IconTab props={auto_tab_props}/>
        <div class="spacer"></div>
        <Time/>
        <div class="spacer"></div>
        <IconTab props={refresh}/>
    </div>
    {#if main !== undefined}
        {#if main.field === MainField.iframe && main.iframe_meta !== undefined}
            <iframe class="full-width" src={main.iframe_meta.url} title={main.iframe_meta.title}>
                <p>iframe unsupported</p>
            </iframe>
            <div class="hide-cursor"></div>
        {:else if main.field === MainField.component && main.component_meta !== undefined}
            {#if main.component_meta === ComponentType.clock}
                <TimerPage timers={timers}/>
            {/if}
        {/if}
    {/if}
</div>

<style>
    .full-width
    {
        width:100%;
        flex-grow: 1;
    }
    .main
    {
        width: 100vw;
        height:100vh;
        margin: 0px;
        display:flex;
        flex-direction: column;
    }
    .tab-row
    {
        width: 100%;
        /*Make height absolute for touch device*/
        height: 16mm;
        display:flex;
        flex-direction: row;
    }
    .spacer
    {
        flex-shrink: true;
        width:100%;
    }
    * {
        color-scheme: dark;
    }
    /*This is for kiosks, so hide the cursor*/
    /*Doesn't work with iframe!*/
    /*
    * {
        cursor: none;
    }
    */
</style>