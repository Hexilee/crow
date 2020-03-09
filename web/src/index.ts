import * as THREE from 'three'
import Stats from 'stats.js'
import { OrbitControls, MapControls } from 'three/examples/jsm/controls/OrbitControls'
import { TransformControls } from 'three/examples/jsm/controls/TransformControls'
import { DelegateCurve } from './curve'
import { Vector3 } from 'three'

const camera = new THREE.PerspectiveCamera(36, window.innerWidth / window.innerHeight, 0.25, 16)
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

const object = new THREE.Mesh()
object.material = new THREE.MeshPhongMaterial({
    color: 0x80ee10,
    shininess: 100,
    side: THREE.DoubleSide,
})
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

const socket = new WebSocket('ws://localhost:8000')
socket.addEventListener('open', event => {
    socket.send("Hello, Server")
})
socket.addEventListener('message', event => {
    let data = JSON.parse(event.data) as Curve
    const curve = new THREE.CatmullRomCurve3(data.points.map(
        ({x, y, z}) => (new Vector3(x, y, z)),
    ))
    object.geometry = new THREE.TubeGeometry(
        curve,  //path
        64,
        0.4,
    )
    console.log('set geometry')
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

// let hiding = 0;
const transformControl = new TransformControls(camera, renderer.domElement)
transformControl.addEventListener('change', render)
transformControl.addEventListener('dragging-changed', (event) => {
    controls.enabled = !event.value
})
transformControl.attach(object)
transformControl.setMode('translate')
scene.add(transformControl)

const init = () => {
    camera.position.set(0, 1.3, 3)
    // Lights
    scene.add(new THREE.AmbientLight(0x505050))
    let spotLight = new THREE.SpotLight(0xffffff)
    spotLight.angle = Math.PI / 5
    spotLight.penumbra = 0.2
    spotLight.position.set(10, 15, 15)
    spotLight.castShadow = true
    spotLight.shadow.camera.near = 3
    spotLight.shadow.camera.far = 10
    spotLight.shadow.mapSize.width = 1024
    spotLight.shadow.mapSize.height = 1024
    scene.add(spotLight)
    let dirLight = new THREE.DirectionalLight(0x55505a, 1)
    dirLight.position.set(0, 15, 0)
    dirLight.castShadow = true
    dirLight.shadow.camera.near = 1
    dirLight.shadow.camera.far = 10
    dirLight.shadow.camera.right = 1
    dirLight.shadow.camera.left = -1
    dirLight.shadow.camera.top = 1
    dirLight.shadow.camera.bottom = -1
    dirLight.shadow.mapSize.width = 1024
    dirLight.shadow.mapSize.height = 1024
    scene.add(dirLight)
    scene.add(object)

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
    renderer.render(scene, camera)
    stats.end()
}
animate()
