// interface Foo {}
import { redirect } from "ext:filter_ext";
redirect("hello redirect");
function http_request_header_filter(req: RoffRequest) {}
function http_response_body_filter(req: RoffRequest, resp: RoffResponse) {}

export { http_request_header_filter, http_response_body_filter };
