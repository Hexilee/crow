import * as THREE from "three"

const size = 20
const divisions = 100
const helperX = new THREE.GridHelper(size, divisions)
const helperY = new THREE.GridHelper(size, divisions)
const helperZ = new THREE.GridHelper(size, divisions)
helperX.rotateZ(Math.PI / 2)
helperZ.rotateX(Math.PI / 2)
if (helperX.material instanceof THREE.Material) {
    helperX.material.opacity = 0.5;
}
if (helperY.material instanceof THREE.Material) {
    helperY.material.opacity = 0.5;
}
if (helperZ.material instanceof THREE.Material) {
    helperZ.material.opacity = 0.5;
}

let isSet = false
export const setGrid = (scene: THREE.Scene) => {
    scene.add(helperX, helperY, helperZ)
    isSet = true
}
export const removeGrid = (scene: THREE.Scene) => {
    if (isSet) {
        scene.remove(helperX, helperY, helperZ)
        isSet = false
    }
}