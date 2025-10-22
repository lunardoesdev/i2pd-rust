import { invoke } from "@tauri-apps/api/core";

class I2PD {
    i2pd_initialised: boolean
    args: string
    datadir: string

    constructor(cfg: {
        datadir: string
    }) {
        this.i2pd_initialised = false
        this.datadir = cfg.datadir
        this.args = "i2pd --datadir=" + this.datadir
    }
    
    private async init(args: string) {
        return await invoke("i2pd_init", {
            args: args
        })
    }

    async start() {
        if (!this.i2pd_initialised) {
            this.init(this.args)
            this.i2pd_initialised = true
        }
        return await invoke("i2pd_start")
    }

    async restart() {
        await this.stop()
        await this.terminate()
        await this.start()
    }

    private async stop() {
        return await invoke("i2pd_stop")
    }

    private async terminate() {
        return await invoke("i2pd_terminate")
    }
 
}

const i2pd = new I2PD({
    datadir: "/home/satori/dist"
})

export default i2pd