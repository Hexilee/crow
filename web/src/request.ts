import { CurveServiceClient } from './gen/grpc/CrowServiceClientPb'
import { Curve, CurveRequest } from './gen/grpc/crow_pb'
import { ClientReadableStream } from 'grpc-web'

export abstract class CurveReactor {
    private _hostname: string
    private _credentials: null | { [index: string]: string; }
    private _options: null | { [index: string]: string; }
    protected client: CurveServiceClient
    protected channel: ClientReadableStream<Curve>
    public request: CurveRequest

    protected constructor(hostname: string, request: CurveRequest,
                          credentials?: { [index: string]: string; },
                          options?: { [index: string]: string; }) {
        this._hostname = hostname
        this._credentials = credentials || null
        this._options = options || null
        this.request = request
        this.updateChannel()
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

    updateClient() {
        this.channel.cancel()
        this.client = new CurveServiceClient(this.hostname, this.credentials, this.options)
    }

    sendRequest(): ClientReadableStream<Curve> {
        return this.client.getCurve(this.request)
    }

    updateChannel() {
        this.client = new CurveServiceClient(this.hostname, this.credentials, this.options)
        this.channel = this.sendRequest()
    }
}
