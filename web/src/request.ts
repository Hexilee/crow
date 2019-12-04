import { CurveServiceClient } from './gen/grpc/CrowServiceClientPb'
import { Curve, CurveRequest } from './gen/grpc/crow_pb'
import { ClientReadableStream, Error, Status } from 'grpc-web'

export abstract class CurveReactor {
    _hostname: string
    _credentials: null | { [index: string]: string; }
    _options: null | { [index: string]: string; }
    _request: CurveRequest
    client: CurveServiceClient
    channel: ClientReadableStream<Curve>

    protected constructor(hostname: string, request: CurveRequest,
                          credentials?: { [index: string]: string; },
                          options?: { [index: string]: string; }) {
        this._hostname = hostname
        this._credentials = credentials || null
        this._options = options || null
        this._request = request
        this.client = new CurveServiceClient(this.hostname, this.credentials, this.options)
        this.channel = this.sendRequest()
        this.setCallback()
    }

    get hostname(): string {
        return this._hostname
    }

    set hostname(hostname: string) {
        this._hostname = hostname
        this.updateClient()
    }

    get credentials(): { [p: string]: string } | null {
        return this._credentials
    }

    set credentials(value: { [p: string]: string } | null) {
        this._credentials = value
        this.updateClient()
    }

    get options(): { [p: string]: string } | null {
        return this._options
    }

    set options(value: { [p: string]: string } | null) {
        this._options = value
        this.updateClient()
    }

    get request(): CurveRequest {
        return this._request
    }

    set request(value: CurveRequest) {
        this._request = value
        this.updateChannel()
    }

    updateClient() {
        this.client = new CurveServiceClient(this.hostname, this.credentials, this.options)
        this.updateChannel()
    }

    updateChannel() {
        this.channel.cancel()
        this.channel = this.sendRequest()
        this.setCallback()
    }

    sendRequest(): ClientReadableStream<Curve> {
        return this.client.getCurve(this.request)
    }

    setCallback() {
        this.channel.on('data', this.onData)
        this.channel.on('status', this.onStatus)
        this.channel.on('error', this.onError)
        this.channel.on('end', this.onEnd)
    }

    abstract onData(data: Curve): void

    abstract onStatus(stat: Status): void

    abstract onError(err: Error): void

    abstract onEnd(): void
}
