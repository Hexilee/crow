import * as THREE from 'three'
import Stats from 'stats.js'
import {OrbitControls} from 'three/examples/jsm/controls/OrbitControls'
import {config} from './config'
import * as tube from './tube'
import * as line from './line'
import {removeAxes, setAxes} from "./helper/axes"
import {removeGrid, setGrid} from "./helper/grid"

const camera = new THREE.PerspectiveCamera(70, window.innerWidth / window.innerHeight)
const scene = new THREE.Scene()
const renderer = new THREE.WebGLRenderer()
const stats = new Stats()
// Geometry

// const controls = new MapControls(camera, renderer.domElement)
// controls.update()
// controls.enableDamping = true // an animation loop is required when either damping or auto-rotation are enabled
// controls.dampingFactor = 0.05
// controls.screenSpacePanning = false
// controls.maxPolarAngle = Math.PI / 8

const orbitControls = new OrbitControls(camera, renderer.domElement)
orbitControls.target = new THREE.Vector3(0, 0, 0)
orbitControls.autoRotate = false
orbitControls.enableDamping = true // an animation loop is required when either damping or auto-rotation are enabled
orbitControls.dampingFactor = 0.05
// orbitControls.screenSpacePanning = false
// orbitControls.maxPolarAngle = Math.PI / 8

const render = () => {
    orbitControls.update();
    renderer.render(scene, camera)
}

// const transformControl = new TransformControls(camera, renderer.domElement)
// transformControl.addEventListener('change', render)
// transformControl.addEventListener('dragging-changed', (event) => {
//     controls.enabled = !event.value
// })
// transformControl.attach(object)
// transformControl.setMode('translate')
// scene.add(transformControl)

const init = () => {
    camera.position.set(10, 10, 0)
    // Lights
    scene.add(new THREE.AmbientLight(0x111111))
    let spotLight = new THREE.DirectionalLight(0x505050, 1.5)
    spotLight.position.set(0, 1000, 0)
    spotLight.castShadow = true
    spotLight.shadow.camera.near = 3
    spotLight.shadow.camera.far = 10
    spotLight.shadow.mapSize.width = 1024
    spotLight.shadow.mapSize.height = 1024
    scene.add(spotLight)
    // const planeGeometry = new THREE.PlaneBufferGeometry(200, 200)
    // planeGeometry.rotateX(-Math.PI / 2)
    // const planeMaterial = new THREE.ShadowMaterial({opacity: 1, color: 0xf0f0f0})
    // const plane = new THREE.Mesh(planeGeometry, planeMaterial)
    // plane.position.y = 0
    // plane.receiveShadow = true
    // scene.add(plane)

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
const animate = () => {
    requestAnimationFrame(animate)
    stats.begin()
    scene.background = new THREE.Color(config.backgroundColor)
    if (config.mode == 'tube') {
        scene.remove(line.object)
        tube.update()
        scene.add(tube.object)
    } else {
        scene.remove(tube.object)
        line.update()
        scene.add(line.object)
    }

    if (config.axes) {
        setAxes(scene)
    } else {
        removeAxes(scene)
    }

    if (config.grid) {
        setGrid(scene)
    } else {
        removeGrid(scene)
    }

    render()
    stats.end()
}
animate()
