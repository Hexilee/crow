import * as THREE from 'three'
import {curve} from './curve'
import {config} from "./config"

const material = new THREE.MeshPhongMaterial({
    shininess: 100,
    side: THREE.DoubleSide,
})
const bufGeometry = new THREE.BufferGeometry()
export const object = new THREE.Mesh(bufGeometry, material)
export const update = () => {
    material.setValues({color: config.color})
    if (curve !== null) {
        let geometry = new THREE.TubeGeometry(
            curve,
            64,
            0.1,
        )
        bufGeometry.fromGeometry(geometry)
        geometry.dispose()
    }
}
object.castShadow = true

