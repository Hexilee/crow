import { GUI } from 'dat.gui'

interface Config {
    realMode: boolean
}

export const config: Config = {
    realMode: true,
}

const gui = new GUI()
gui.add(config, 'realMode')
