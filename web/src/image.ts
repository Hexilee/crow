import * as THREE from 'three'
import { curve } from './curve'

const material = new THREE.LineBasicMaterial({color: 0x80ee10})
const bufGeometry = new THREE.BufferGeometry()
export const backgroundColor = new THREE.Color(0xFFFFFF)
export const object = new THREE.Line(bufGeometry, material)
export const updateGeometry = () => {
    if (curve !== null) {
        bufGeometry.setFromPoints(curve.getPoints(100))
    }
}
object.castShadow = true
