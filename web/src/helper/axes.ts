import * as THREE from "three"

const o = new THREE.Vector3(0, 0, 0)
const x = new THREE.Vector3(1, 0, 0)
const y = new THREE.Vector3(0, 1, 0)
const z = new THREE.Vector3(0, 0, 1)
const ArrayX = new THREE.ArrowHelper(x, o, 15, 0xff0000, 1, 0.5)
const ArrayY = new THREE.ArrowHelper(y, o, 15, 0x00ff00, 1, 0.5)
const ArrayZ = new THREE.ArrowHelper(z, o, 15, 0x0000ff, 1, 0.5)

let isSet = false
export const setAxes = (scene: THREE.Scene) => {
    scene.add(ArrayX, ArrayY, ArrayZ)
    isSet = true
}
export const removeAxes = (scene: THREE.Scene) => {
    if (isSet) {
        scene.remove(ArrayX, ArrayY, ArrayZ)
        isSet = false
    }
}