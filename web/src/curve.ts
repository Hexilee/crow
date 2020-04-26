import * as THREE from 'three'
import { Vector3 } from 'three'

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
    const socket = new WebSocket(process.env.WS_URL)
    socket.addEventListener('open', event => {
        socket.send('Hello, Server')
    })
    socket.addEventListener('message', event => {
        let data = JSON.parse(event.data) as Curve
        curve = new THREE.CatmullRomCurve3(data.points.map(
            ({x, y, z}) => (new Vector3(x, y, z)),
        ))
    })

    socket.addEventListener('error', event => {
    })

    socket.addEventListener('close', event => {
    })
}