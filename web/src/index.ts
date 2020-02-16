import * as THREE from 'three'
import Stats from 'stats.js'
import { OrbitControls } from 'three-orbitcontrols-ts'
// import { TransformControls } from './types/three-transformcontrols';

const camera = new THREE.PerspectiveCamera(36, window.innerWidth / window.innerHeight, 0.25, 16)
const scene = new THREE.Scene()
const renderer = new THREE.WebGLRenderer()
const stats = new Stats()
// Geometry
const material = new THREE.MeshPhongMaterial({
    color: 0x80ee10,
    shininess: 100,
    side: THREE.DoubleSide,
})

const curve = new THREE.CatmullRomCurve3([
    new THREE.Vector3(0.0004999916667219883, 0.0004999916667219883, 0.09999666669999985),
    new THREE.Vector3(0.0004999916667223718, 0.000499991666722913, 0.0000000000000001110223024625156),
    new THREE.Vector3(0.0009998833401111246, 0.0009998833401109347, -0.09998333469994203),
    new THREE.Vector3(0.002998950223639105, 0.0029989502236391725, -0.19991668989623468),
    new THREE.Vector3(0.0074937781731317255, 0.007493778173131816, -0.299673543714417),
    new THREE.Vector3(0.015471670662337636, 0.015471670662337695, -0.398978026986938),
    new THREE.Vector3(0.027896630394832853, 0.02789663039483286, -0.49734682562591237),
    new THREE.Vector3(0.04568509121803388, 0.04568509121803349, -0.5940344476842224),
    new THREE.Vector3(0.06966889620691101, 0.06966889620691076, -0.6879852544999636),
    new THREE.Vector3(0.1005436345107183, 0.10054363451071825, -0.7777973713859363),
    new THREE.Vector3(0.13846926001477475, 0.1379700137265746, -0.8623735998164869),
    new THREE.Vector3(0.18439073824915822, 0.18389140873491924, -0.9383698014168048),
    new THREE.Vector3(0.23751342088321314, 0.2365143965198983, -1.0047201437905136),
    new THREE.Vector3(0.297045393943732, 0.2940566523551529, -1.0607453068746413),
    new THREE.Vector3(0.3622266988168996, 0.3547900560777047, -1.1061004582392833),
    new THREE.Vector3(0.4323482754725529, 0.4170685070079855, -1.1407224638472933),
    new THREE.Vector3(0.5067617299611967, 0.4793430352646434, -1.1647799749670837),
    new THREE.Vector3(0.5848811797360163, 0.5401669933164821, -1.1786288882252025),
    new THREE.Vector3(0.6661784494126711, 0.5981941062263777, -1.1827747017795487),
    new THREE.Vector3(0.7501728260537689, 0.6521720267283319, -1.177842496188031),
    new THREE.Vector3(0.8360495682472028, 0.7015998734024762, -1.1643950530765028),
    new THREE.Vector3(0.924145400591597, 0.7440720758282711, -1.1435469353459966),
    new THREE.Vector3(1.0133019949443487, 0.7799043598099622, -1.1158507564970606),
    new THREE.Vector3(1.1025532444401, 0.8095638995052822, -1.0818676514471255),
    new THREE.Vector3(1.1911157776126085, 0.8336340562705274, -1.0421448481962172),
    new THREE.Vector3(1.2783737108632711, 0.8527843810413527, -0.9971997807738272),
    new THREE.Vector3(1.363859659131192, 0.8677460500371804, -0.9475101265397731),
    new THREE.Vector3(1.447233636591653, 0.8792925246661887, -0.8935090734751284),
    new THREE.Vector3(1.5282611073446968, 0.8882249531481947, -0.8355851183743105),
    new THREE.Vector3(1.6067911121932978, 0.8953616439254672, -0.7740857404632734),
    new THREE.Vector3(1.6827351143924054, 0.9015308194815034, -0.7093243656714779),
    new THREE.Vector3(1.7560469804196839, 0.9075657854710683, -0.6415901166651317),
    new THREE.Vector3(1.8267043422772695, 0.9143016087272756, -0.5711599180513043),
    new THREE.Vector3(1.8946914733325724, 0.9225723770709642, -0.49831258289877),
    new THREE.Vector3(1.959983746127137, 0.9332081063191162, -0.4233445362302899),
    new THREE.Vector3(2.022901832332199, 0.9464166847969112, -0.34673599378565434),
    new THREE.Vector3(2.0826499110192063, 0.9642395159354398, -0.26854169136982525),
    new THREE.Vector3(2.1398416520247254, 0.9862352898162083, -0.1895052033417869),
    new THREE.Vector3(2.1951689964292, 1.0119761390676587, -0.11027619087984826),
    new THREE.Vector3(2.2493787384744466, 1.041048850514198, -0.03143256316238023),
    new THREE.Vector3(2.3032520580198406, 1.0730520331415185, 0.0464957257061025),
    new THREE.Vector3(2.357585988396269, 1.107589922811456, 0.12300233657815),
    new THREE.Vector3(2.413175670825788, 1.1442634732328283, 0.19758058341872872),
    new THREE.Vector3(2.4707961861156287, 1.1826593666630456, 0.2697006859478905),
    new THREE.Vector3(2.531182761680664, 1.2223375911976853, 0.3387890732342918),
    new THREE.Vector3(2.595008229321131, 1.2628182845602403, 0.40421065427543207),
    new THREE.Vector3(2.6628567663738902, 1.3035686428724989, 0.46525511851289303),
    new THREE.Vector3(2.7351932063767888, 1.3439908378306924, 0.5211285249260658),
    new THREE.Vector3(2.812327576716522, 1.3834120719131555, 0.5709516563270242),
    new THREE.Vector3(2.8943750334434872, 1.4210781161553774, 0.6137668196065665),
    new THREE.Vector3(2.980889936786012, 1.4563827618547809, 0.6493489490568896),
    new THREE.Vector3(3.0722001145725093, 1.4879796693083422, 0.675080983325217),
    new THREE.Vector3(3.166959278080135, 1.5153575750209545, 0.6915060373895277),
    new THREE.Vector3(3.2640126899935598, 1.5381282786695245, 0.699329475390047),
    new THREE.Vector3(3.3623983604015395, 1.5560068389368449, 0.6993716835638029),
    new THREE.Vector3(3.461339156098751, 1.5687918930988598, 0.6925292128242985),
    new THREE.Vector3(3.560228277223969, 1.5763472430420276, 0.679744299032732),
    new THREE.Vector3(3.6586101778512794, 1.5785855355322391, 0.6619822839852161),
    new THREE.Vector3(3.7561586054801737, 1.5754546078558689, 0.6402161325511584),
    new THREE.Vector3(3.8526530464529616, 1.5669268718258762, 0.6154170378870095),
    new THREE.Vector3(3.947954515823724, 1.5529919644359056, 0.5885499943509532),
    new THREE.Vector3(4.041981336957821, 1.5336527932926391, 0.5605731669398658),
    new THREE.Vector3(4.134685326701556, 1.5089250382044879, 0.5324398717815909),
    new THREE.Vector3(4.226028639806535, 1.4788401244858462, 0.5051019850423414),
    new THREE.Vector3(4.315961431629444, 1.443451645526796, 0.47951360408673854),
    new THREE.Vector3(4.404400469177635, 1.4028451686273722, 0.4566337874325929),
    new THREE.Vector3(4.491208854327788, 1.357151295752455, 0.4374271975924688),
    new THREE.Vector3(4.57617711550613, 1.3065617569330017, 0.4228614679388026),
    new THREE.Vector3(4.659006070020669, 1.251348176688988, 0.41390012193954867),
    new THREE.Vector3(4.739292051336596, 1.1918829629915861, 0.41148990711963773),
    new THREE.Vector3(4.8165025644413975, 1.1285275644321677, 0.41587183462171506),
    new THREE.Vector3(4.89005980552883, 1.0621536457421021, 0.42924189562406984),
    new THREE.Vector3(4.959182506972023, 0.9932843255717992, 0.45102049376516273),
    new THREE.Vector3(5.02320115501423, 0.9224016976144255, 0.48056338248533464),
    new THREE.Vector3(5.0815523012102775, 0.8499433655382099, 0.5171788351636067),
    new THREE.Vector3(5.1337715134887905, 0.7763007073561731, 0.5601424737434548),
    new THREE.Vector3(5.17948548354187, 0.7018186216443956, 0.6087097489425981),
    new THREE.Vector3(5.21840373920349, 0.6267965165935829, 0.6621261447530017),
    new THREE.Vector3(5.250310342556505, 0.5514903184186383, 0.7196352412436721),
    new THREE.Vector3(5.27505588929203, 0.4761152958185358, 0.7804848149797361),
    new THREE.Vector3(5.292550064084632, 0.40084951998246987, 0.8439311881334902),
    new THREE.Vector3(5.302754951479206, 0.3258378034310605, 0.9092420580546876),
    new THREE.Vector3(5.305679252454244, 0.2511959844400891, 0.9756980510331181),
    new THREE.Vector3(5.301373513394912, 0.1770154458881987, 1.0425932493578332),
    new THREE.Vector3(5.289926436271581, 0.1033677773242914, 1.1092349414126734),
    new THREE.Vector3(5.2714623056889955, 0.03030950630642463, 1.1749428419935288),
    new THREE.Vector3(5.2461395392721775, -0.04211316075696501, 1.239048025505246),
    new THREE.Vector3(5.214150341587906, -0.11385963716449989, 1.3008918090709722),
    new THREE.Vector3(5.1757214174106, -0.1848903353381135, 1.359824816401772),
    new THREE.Vector3(5.131115676570155, -0.2551621645665959, 1.4152064467338847),
    new THREE.Vector3(5.080634838857321, -0.3246241839445202, 1.466404966125982),
    new THREE.Vector3(5.024622822595942, -0.39321345003644836, 1.5127984304891648),
    new THREE.Vector3(4.963469773750137, -0.4608511045207485, 1.553776640171696),
    new THREE.Vector3(4.897616563241794, -0.5274387555790123, 1.5887443137355102),
    new THREE.Vector3(4.827559548178005, -0.5928552177332221, 1.617125652486768),
    new THREE.Vector3(4.753855357902612, -0.6569536877260138, 1.638370445885591),
    new THREE.Vector3(4.6771254285303465, -0.719559448297952, 1.6519618395159164),
    new THREE.Vector3(4.5980599706663074, -0.780468206599283, 1.6574258500925987),
    new THREE.Vector3(4.517421015627964, -0.8394451885713767, 1.654342664242667),
    new THREE.Vector3(4.4360441475022805, -0.8962251238427195, 1.6423596978283668),
])

const geometry = new THREE.TubeGeometry(
    curve,  //path
    64,
    0.1
);

const object = new THREE.Mesh(geometry, material)
object.castShadow = true
init()
const startTime = Date.now()
const orbitControls = new OrbitControls(camera, renderer.domElement)
orbitControls.target.set(0, 1, 0)
orbitControls.enableDamping = true

// let hiding = 0;
// const transformControl = new TransformControls(camera, renderer.domElement);
// transformControl.addEventListener('change', () => { renderer.render(scene, camera) });
// transformControl.addEventListener('dragging-changed', function (event) {
//     orbitControls.enabled = !event.value;
// });
// scene.add(transformControl);

// // Hiding transform situation is a little in a mess :()
// transformControl.addEventListener('change', function () {
//     cancelHideTransform();
// });

// transformControl.addEventListener('mouseDown', function () {
//     cancelHideTransform();

// })

// transformControl.addEventListener('mouseUp', function () {
//     delayHideTransform();
// });


// const delayHideTransform = () => {
//     cancelHideTransform();
//     hideTransform();
// }

// function hideTransform() {
//     hiding = setTimeout(() => {
//         transformControl.detach(transformControl.object)
//     }, 2500)
// }

// const cancelHideTransform = () => {
//     if (hiding) clearTimeout(hiding);
// }

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
    // const planeGeometry = new THREE.PlaneBufferGeometry(2000, 2000);
    // planeGeometry.rotateX(- Math.PI / 2);
    // const planeMaterial = new THREE.ShadowMaterial({ opacity: 0.2 });

    // const plane = new THREE.Mesh(planeGeometry, planeMaterial);
    // plane.position.y = - 2;
    // plane.receiveShadow = true;
    // scene.add(plane);
    // let ground = new THREE.Mesh(
    //     new THREE.PlaneBufferGeometry(9, 9, 1, 1),
    //     new THREE.MeshPhongMaterial({ color: 0xa0adaf, shininess: 150 }),
    // )
    // ground.rotation.x = -Math.PI / 2 // rotates X/Y to X/Z
    // ground.receiveShadow = true
    // scene.add(ground)
    // Stats
    document.body.appendChild(stats.dom)
    // Renderer
    renderer.shadowMap.enabled = true
    renderer.setPixelRatio(window.devicePixelRatio)
    renderer.setSize(window.innerWidth, window.innerHeight)
    window.addEventListener('resize', onWindowResize, false)
    document.body.appendChild(renderer.domElement)
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight
    camera.updateProjectionMatrix()
    renderer.setSize(window.innerWidth, window.innerHeight)
}

function animate() {
    let currentTime = Date.now()
    let time = (currentTime - startTime) / 1000
    // Controls
    requestAnimationFrame(animate)
    orbitControls.update()
    object.position.y = 0.8
    object.rotation.x = time * 0.5
    object.rotation.y = time * 0.2
    object.scale.setScalar(Math.cos(time) * 0.125 + 0.875)

    stats.begin()
    renderer.render(scene, camera)
    stats.end()
}
