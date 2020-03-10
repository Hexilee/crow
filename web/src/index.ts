import * as THREE from 'three'
import Stats from 'stats.js'
import { OrbitControls, MapControls } from 'three/examples/jsm/controls/OrbitControls'
import { TransformControls } from 'three/examples/jsm/controls/TransformControls'
import { Vector3 } from 'three'

const camera = new THREE.PerspectiveCamera(70, window.innerWidth / window.innerHeight)
const scene = new THREE.Scene()
const renderer = new THREE.WebGLRenderer()
const render = () => {
    renderer.render(scene, camera)
}
const stats = new Stats()
// Geometry
const material = new THREE.MeshPhongMaterial({
    color: 0x80ee10,
    shininess: 100,
    side: THREE.DoubleSide,
})

const bufGeometry = new THREE.BufferGeometry()
const object = new THREE.Mesh(bufGeometry, material)
object.castShadow = true

interface Point {
    readonly x: number,
    readonly y: number,
    readonly z: number,
}

interface Curve {
    readonly timestamp: number,
    readonly points: Array<Point>,
}

let curve: THREE.Curve<Vector3> | null = null

const socket = new WebSocket('ws://localhost:8000')
socket.addEventListener('open', event => {
    socket.send('Hello, Server')
})
socket.addEventListener('message', event => {
    let data = JSON.parse(event.data) as Curve
    curve = new THREE.CatmullRomCurve3(data.points.map(
        ({x, y, z}) => (new Vector3(x, y, z)),
    ))
    console.log('set points')
})

socket.addEventListener('error', event => {
    console.log('receive error')
})

socket.addEventListener('close', event => {
    console.log('socket close')
})


const controls = new MapControls(camera, renderer.domElement)
controls.update()
controls.enableDamping = true // an animation loop is required when either damping or auto-rotation are enabled
controls.dampingFactor = 0.05
controls.screenSpacePanning = false
controls.maxPolarAngle = Math.PI / 8

const transformControl = new TransformControls(camera, renderer.domElement)
transformControl.addEventListener('change', render)
transformControl.addEventListener('dragging-changed', (event) => {
    controls.enabled = !event.value
})
transformControl.attach(object)
transformControl.setMode('translate')
scene.add(transformControl)

const init = () => {
    camera.position.set(10, 10, 0)
    // Lights
    scene.add(new THREE.AmbientLight(0xf0f0f0))
    let spotLight = new THREE.DirectionalLight(0x505050, 1.5)
    spotLight.position.set(0, 1000, 0)
    spotLight.castShadow = true
    spotLight.shadow.camera.near = 3
    spotLight.shadow.camera.far = 10
    spotLight.shadow.mapSize.width = 1024
    spotLight.shadow.mapSize.height = 1024
    scene.add(spotLight)
    scene.add(object)
    const planeGeometry = new THREE.PlaneBufferGeometry(2000, 2000)
    // planeGeometry.rotateX(-Math.PI / 2)
    const planeMaterial = new THREE.ShadowMaterial({opacity: 0.2, color: 0xf0f0f0})
    const plane = new THREE.Mesh(planeGeometry, planeMaterial)
    plane.position.y = -2
    plane.receiveShadow = true
    scene.add(plane)

    document.body.appendChild(stats.dom)
    renderer.shadowMap.enabled = true
    renderer.setPixelRatio(window.devicePixelRatio)
    renderer.setSize(window.innerWidth, window.innerHeight)
    window.addEventListener('resize', onWindowResize, false)
    document.body.appendChild(renderer.domElement)
}

const onWindowResize = () => {
    camera.aspect = window.innerWidth / window.innerHeight
    camera.updateProjectionMatrix()
    renderer.setSize(window.innerWidth, window.innerHeight)
    render()
}

init()
render()
const animate = () => {
    requestAnimationFrame(animate)
    stats.begin()
    if (curve !== null) {
        let geometry = new THREE.TubeGeometry(
            curve,
            64,
            0.2,
        )
        bufGeometry.fromGeometry(geometry)
        geometry.dispose()
    }
    renderer.render(scene, camera)
    stats.end()
}
animate()
