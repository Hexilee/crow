import * as THREE from 'three'
import { curve } from './curve'

const material = new THREE.MeshPhongMaterial({
    color: 0x80ee10,
    shininess: 100,
    side: THREE.DoubleSide,
})
const bufGeometry = new THREE.BufferGeometry()
export const backgroundColor = new THREE.Color(0x000000)
export const object = new THREE.Mesh(bufGeometry, material)
export const updateGeometry = () => {
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

