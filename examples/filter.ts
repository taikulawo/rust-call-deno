async function http_request_header_filter(req: BridgeRequest) {}
async function http_response_body_filter(
    req: BridgeRequest,
    resp: BridgeResponse
) {}

export { http_request_header_filter, http_response_body_filter };
