## Realtime 3D curve reconstruction

### Deploy

#### Frontend

Refer to [Frontend](https://github.com/Hexilee/crow/tree/master/web), there is a deployment on
[https://curve.hexilee.me:8000/](https://curve.hexilee.me:8000/).

#### Backend

Refer to [Backend](https://github.com/Hexilee/crow/tree/master/server), there is a deployment on
[https://curve.hexilee.me:8000/](https://curve.hexilee.me:8000/ws).


### Data Source

Register a raw data source by a websocket query on `<base_url>/upstream`,
 for example:

```javascript
let source = new WebSocket("wss://curve.hexilee.me:8000/ws/upstream")
source.addEventListener('open', event => {
    socket.send('Hello, Server')
})
source.addEventListener('message', event => {
    console.log(event.data)
})
```

Get source id by response: `{"id": xxx}`, then input id to `channel` option on frontend.

Now, you can send raw data to server:

```javascript
source.send("[[0,0,0],[4.66,0.21,0],[9.36,0.27,0],[14.82,0.086,0],[19.72,-0.0093,0],[24.74,-0.091,0],[29.95,-0.079,0]]")
```

### Subscribe Channel

If you want to subscribe channel by yourself (instead of frontend webpage), run the following script in browser console:

```javascript
let lib = document.createElement('script')
lib.type = 'text/javascript'
lib.src = 'https://unpkg.com/pako@1.0.11/dist/pako.min.js'
document.head.appendChild(lib)

let socket = new WebSocket("wss://curve.hexilee.me:8000/ws/downstream/2") // replace 2 with the id you want

socket.addEventListener('message', event => {
    if (event.data instanceof Blob) {
        event.data.arrayBuffer().then(buffer => {
            let json = pako.inflateRaw(new Uint8Array(buffer), { to: 'string' })
            console.log(json)
        }).catch(err => {
            console.log(err)
        })
    }
})
```