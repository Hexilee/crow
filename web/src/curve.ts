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

export let curve: THREE.Curve<Vector3> | null = null

if (process.env.WS_URL !== undefined) {
    const socket = new WebSocket(`${process.env.WS_URL}/down/0`)
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