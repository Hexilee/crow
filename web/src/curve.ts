import { Curve } from 'three/src/extras/core/Curve'
import { Vector3 } from 'three/src/math/Vector3'

export class DelegateCurve extends Curve<Vector3> {
    constructor(curve: Curve<Vector3>) {
        super()
        this.curve = curve
    }

    curve: Curve<Vector3>

    getPoint(t: number): Vector3 {
        return this.curve.getPoint(t)
    }

    setCurve(curve: Curve<Vector3>) {
        this.curve = curve
    }
}