import {GUI} from 'dat.gui'

interface Config {
    mode: 'tube' | 'line',
    color: string
    backgroundColor: string
}

export const config: Config = {
    mode: 'tube',
    color: '#FF4700',
    backgroundColor: '#000000',
}

const gui = new GUI()
gui.add(config, 'mode', ['tube', 'line'])
gui.add(config, 'color')
gui.add(config, 'backgroundColor')

