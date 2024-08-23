type HttpMethod = "get" | "post" | "delete" | "put";
declare class RoffRequest {
    url(): string;
    path(): string;
    get_header(k: String): string;
    redirect(s: String);
    method(): HttpMethod;
}
declare class RoffResponse {
    body_len: number;
    body(): ArrayBuffer;
    set_header(k: String, v: String);
    get_header(k: String): string;
}
