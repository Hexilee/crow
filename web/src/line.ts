import * as THREE from 'three'
import {curve} from './curve'
import {config} from "./config"

const material = new THREE.LineBasicMaterial()
const bufGeometry = new THREE.BufferGeometry()
export const object = new THREE.Line(bufGeometry, material)
export const update = () => {
    material.setValues({color: config.color})
    if (curve !== null) {
        bufGeometry.setFromPoints(curve.getPoints(100))
    }
}
object.castShadow = true
