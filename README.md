### Realtime 3D curve reconstruction

- [Protocol](https://github.com/Hexilee/crow/tree/master/proto)
- [Frontend](https://github.com/Hexilee/crow/tree/master/web)
- [Mock Server](https://github.com/Hexilee/crow/tree/master/mock)
- [Backend](https://github.com/Hexilee/crow/tree/master/backend)


```js
let data = "[[0,0,0],[4.66,0.21,0],[9.36,0.27,0],[14.82,0.086,0],[19.72,-0.0093,0],[24.74,-0.091,0],[29.95,-0.079,0]]"
let socket = new WebSocket("wss://curve.hexilee.me:8000/ws/upstream")
socket.addEventListener('open', event => {
    socket.send('Hello, Server')
})
socket.addEventListener('message', event => {
    console.log(event.data)
})
setInterval(() => socket.send(data), 1000)
```

