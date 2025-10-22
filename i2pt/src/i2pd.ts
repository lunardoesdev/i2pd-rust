import { invoke } from "@tauri-apps/api/core";
import * as path from '@tauri-apps/api/path';
import * as taurifs from '@tauri-apps/plugin-fs';


class I2PD {
    i2pd_initialised: boolean
    args: string
    datadir: string
    restarting: boolean = false

    constructor(cfg: {
        datadir: string
    }) {
        this.i2pd_initialised = false
        this.datadir = cfg.datadir
        this.args = "i2pd --datadir=" + this.datadir
        this.restarting = false
    }
    
    private async init(args: string) {
        return await invoke("i2pd_init", {
            args: args
        })
    }

    async start() {
        this.restarting = true
        if (!this.i2pd_initialised) {
            this.init(this.args)
            this.i2pd_initialised = true
        }
        await invoke("i2pd_start")
    }

    async restart() {
        if (this.restarting) {
            return
        }
        this.restarting = true
        await this.stop()
        await this.terminate()
        await this.start()
        let x = this
        setTimeout(() => {
            x.restarting = false
        }, 1000)
    }

    private async stop() {
        return await invoke("i2pd_stop")
    }

    private async terminate() {
        return await invoke("i2pd_terminate")
    }

    private static async prepareAssets() {
      const configdir = await path.appConfigDir()
      if (! await taurifs.exists(await path.join(configdir, "i2pdata"))) {
        await invoke('copy_resource_dir_to_config', { resourceDirName: 'i2pdata' })
      }
    }

    private static instance: I2PD | null = null
    public static async getI2pd() {
        await this.prepareAssets();
        if (this.instance == null) {
            this.instance = new I2PD({
                datadir: await path.join(await path.appConfigDir(), "i2pdata")
            })
        }
        return this.instance
    }
}

export default I2PD