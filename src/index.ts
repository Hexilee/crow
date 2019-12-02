import * as THREE from 'three'
import Stats from 'stats.js'
import { GUI } from 'dat.gui'
import { OrbitControls } from 'three-orbitcontrols-ts'

const camera = new THREE.PerspectiveCamera(36, window.innerWidth / window.innerHeight, 0.25, 16)
const scene = new THREE.Scene()
const renderer = new THREE.WebGLRenderer()
const stats = new Stats()

const localPlane = new THREE.Plane(new THREE.Vector3(0, -1, 0), 0.8)
const globalPlane = new THREE.Plane(new THREE.Vector3(-1, 0, 0), 0.1)
// Geometry
const material = new THREE.MeshPhongMaterial({
    color: 0x80ee10,
    shininess: 100,
    side: THREE.DoubleSide,
// ***** Clipping setup (material): *****
    clippingPlanes: [localPlane],
    clipShadows: true,
})
const geometry = new THREE.TorusKnotBufferGeometry(0.4, 0.08, 95, 20)
const object = new THREE.Mesh(geometry, material)
object.castShadow = true


init()
const startTime = Date.now()
animate()

function init() {
    camera.position.set(0, 1.3, 3)
// Lights
    scene.add(new THREE.AmbientLight(0x505050))
    let spotLight = new THREE.SpotLight(0xffffff)
    spotLight.angle = Math.PI / 5
    spotLight.penumbra = 0.2
    spotLight.position.set(2, 3, 3)
    spotLight.castShadow = true
    spotLight.shadow.camera.near = 3
    spotLight.shadow.camera.far = 10
    spotLight.shadow.mapSize.width = 1024
    spotLight.shadow.mapSize.height = 1024
    scene.add(spotLight)
    let dirLight = new THREE.DirectionalLight(0x55505a, 1)
    dirLight.position.set(0, 3, 0)
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
// ***** Clipping planes: *****
    scene.add(object)
    let ground = new THREE.Mesh(
        new THREE.PlaneBufferGeometry(9, 9, 1, 1),
        new THREE.MeshPhongMaterial({color: 0xa0adaf, shininess: 150}),
    )
    ground.rotation.x = -Math.PI / 2 // rotates X/Y to X/Z
    ground.receiveShadow = true
    scene.add(ground)
// Stats
    document.body.appendChild(stats.dom)
// Renderer
    renderer.shadowMap.enabled = true
    renderer.setPixelRatio(window.devicePixelRatio)
    renderer.setSize(window.innerWidth, window.innerHeight)
    window.addEventListener('resize', onWindowResize, false)
    document.body.appendChild(renderer.domElement)
// ***** Clipping setup (renderer): *****
    const globalPlanes = [globalPlane]
    const Empty: any[] = []
    renderer.clippingPlanes = [] // GUI sets it to globalPlanes
    renderer.localClippingEnabled = true
// Controls
    let controls = new OrbitControls(camera, renderer.domElement)
    controls.target.set(0, 1, 0)
    controls.update()
// GUI
    let gui = new GUI(),
        folderLocal = gui.addFolder('Local Clipping'),
        propsLocal = {
            get 'Enabled'() {
                return renderer.localClippingEnabled
            },
            set 'Enabled'(v) {
                renderer.localClippingEnabled = v
            },
            get 'Shadows'() {
                return material.clipShadows
            },
            set 'Shadows'(v) {
                material.clipShadows = v
            },
            get 'Plane'() {
                return localPlane.constant
            },
            set 'Plane'(v) {
                localPlane.constant = v
            },
        },
        folderGlobal = gui.addFolder('Global Clipping'),
        propsGlobal = {
            get 'Enabled'() {
                return renderer.clippingPlanes !== Empty
            },
            set 'Enabled'(v) {
                renderer.clippingPlanes = v ? globalPlanes : Empty
            },
            get 'Plane'() {
                return globalPlane.constant
            },
            set 'Plane'(v) {
                globalPlane.constant = v
            },
        }
    folderLocal.add(propsLocal, 'Enabled')
    folderLocal.add(propsLocal, 'Shadows')
    folderLocal.add(propsLocal, 'Plane', 0.3, 1.25)
    folderGlobal.add(propsGlobal, 'Enabled')
    folderGlobal.add(propsGlobal, 'Plane', -0.4, 3)
// Start
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight
    camera.updateProjectionMatrix()
    renderer.setSize(window.innerWidth, window.innerHeight)
}

function animate() {
    let currentTime = Date.now()
    let time = (currentTime - startTime) / 1000
    requestAnimationFrame(animate)
    object.position.y = 0.8
    object.rotation.x = time * 0.5
    object.rotation.y = time * 0.2
    object.scale.setScalar(Math.cos(time) * 0.125 + 0.875)
    stats.begin()
    renderer.render(scene, camera)
    stats.end()
}
