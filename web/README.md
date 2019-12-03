### Realtime 3D curve reconstruction ---- Frontend

#### Technology Stack

- TypeScript
- WebGL
    - three.js
- grpc-web

#### Development Tools

- protobuf

```bash
> protoc --version
```

- yarn(Recommended)

```bash
> yarn install
```

#### Debug

```bash
> yarn dev
```

#### Release

```bash
> yarn build
```

Then you can serve `dist` fold.

```bash
> yarn serve
```

This script will run a simple `http-server` on `dist` fold.

#### Regenerate GRPC Code

Run after proto files are changed, just:

```bash
> yarn install
```