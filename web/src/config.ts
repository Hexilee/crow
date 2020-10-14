import {GUI} from 'dat.gui'

export interface Config {
    server: string,
    channel: number,
    mode: 'tube' | 'line',
    color: string
    backgroundColor: string
    axes: boolean,
    grid: boolean,
}

export const config: Config = {
    server: process.env.WS_URL || 'ws://127.0.0.1:8000',
    channel: 0,
    mode: 'tube',
    color: '#FF4700',
    backgroundColor: '#000000',
    axes: false,
    grid: false,
}

const gui = new GUI()
gui.add(config, 'server')
gui.add(config, 'channel')
gui.add(config, 'mode', ['tube', 'line'])
gui.add(config, 'color')
gui.add(config, 'backgroundColor')
gui.add(config, 'axes')
gui.add(config, 'grid')

