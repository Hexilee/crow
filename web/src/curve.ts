import * as THREE from 'three'
import { Vector3 } from 'three'
import { inflateRaw } from 'pako'

interface Point {
    readonly x: number,
    readonly y: number,
    readonly z: number,
}

interface Curve {
    readonly timestamp: number,
    readonly points: Array<Point>,
}

let socket: WebSocket

export let curve: THREE.Curve<Vector3> | null = null

export const reconnect = (baseUrl: string, channel: number) => {
    if (socket !== undefined) {
        socket.close()
    }

    socket = new WebSocket(`${baseUrl}/down/${channel}`)
    socket.addEventListener('open', event => {
        socket.send('Hello, Server')
    })
    socket.addEventListener('message', event => {
        if (event.data instanceof Blob) {
            event.data.arrayBuffer().then(buffer => {
                let json = inflateRaw(new Uint8Array(buffer), { to: 'string' })
                let data = JSON.parse(json) as Curve
                curve = new THREE.CatmullRomCurve3(data.points.map(
                    ({ x, y, z }) => (new Vector3(x, y, z)),
                ))
            }).catch(err => {
                console.log(err)
            })
        }
    })

    socket.addEventListener('error', event => {
    })

    socket.addEventListener('close', event => {
    })
}