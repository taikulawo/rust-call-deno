// interface Foo {}
import { redirect } from "ext:filter_ext";
redirect("hello redirect");
type HttpMethod = "get" | "post" | "delete" | "put";
declare class Request {
    url(): string;
    path(): string;
    get_header(k: String): string;
    redirect(s: String);
    method(): HttpMethod;
}
declare class Response {
    body_len: number;
    body(): ArrayBuffer;
    set_header(k: String, v: String);
    get_header(k: String): string;
}
function http_request_header_filter(req: RoffRequest) {}
function http_response_body_filter(req: RoffRequest, resp: RoffResponse) {}

export { http_request_header_filter, http_response_body_filter };
