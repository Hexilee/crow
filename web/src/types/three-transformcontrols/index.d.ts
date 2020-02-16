import { Camera, Object3D } from 'three';

export class TransformControls extends Object3D {
    constructor(object: Camera, domElement?: HTMLElement);

    object: Object3D;

    update(): void;
    detach(object: Object3D): void;
    attach(object: Object3D): TransformControls;
    getMode(): string;
    setMode(mode: string): void;
    setSnap(snap: any): void;
    setSize(size: number): void;
    setSpace(space: string): void;

}