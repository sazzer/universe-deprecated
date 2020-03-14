import * as rfc6570 from "rfc6570-expand";

import { ProblemResponse } from "./problem";
import debug from "debug";
import { is as typeIs } from "type-is";

/** The logger to use */
const LOG = debug("universe:api");

/** The error logger to use */
const ERROR_LOG = debug("universe:api:error");
debug.enable("universe:api:error");

declare global {
  interface Window {
    _UNIVERSE_CONFIG: any;
  }
}

/** The access token to use */
let _accessToken: string | undefined;

/**
 * The shape of the request that we want to make
 */
export interface ApiRequest {
  url: string;
  urlParams?: any;
  method?: string;
  headers?: any;
  data?: any;
  forceReload?: boolean;
}

export interface ApiResponse<T> {
  data: T;
  status: number;
  headers: any;
}

/**
 * Error thrown when something goes unexpectedly wrong on an API call
 */
export class ErrorResponse extends Error {
  /** The API response */
  readonly response: ApiResponse<any>;

  constructor(response: ApiResponse<any>) {
    super("Request failed with status code " + response.status);
    this.response = response;
  }
}
/**
 * Build an API requester to make API calls with
 */
export async function request<T>(request: ApiRequest): Promise<ApiResponse<T>> {
  let serviceUrl = process.env.REACT_APP_SERVICE_URL;
  if (window && window._UNIVERSE_CONFIG && window._UNIVERSE_CONFIG.serviceUrl) {
    serviceUrl = window._UNIVERSE_CONFIG.serviceUrl;
  }

  const template = rfc6570.init(serviceUrl + request.url);
  const expandedUri = template.expand(request.urlParams || {});
  LOG("Making request to: %s", expandedUri);

  const headers = {
    ...request.headers
  };

  if (_accessToken) {
    headers.Authorization = `Bearer ${_accessToken}`;
  }

  const result = await fetch(expandedUri, {
    method: request.method || "GET",
    headers,
    mode: "cors",
    body: JSON.stringify(request.data),
    cache: request.forceReload ? "no-cache" : "default"
  });

  let data;
  const contentType = result.headers.get("Content-Type") || "";
  if (typeIs(contentType, ["application/json", "application/*+json"])) {
    data = await result.json();
  } else {
    data = await result.text();
  }

  if (result.status >= 400) {
    if (typeIs(contentType, ["application/problem+json"])) {
      LOG("Received an RFC-7807 Problem response: %o", data);
      throw new ProblemResponse(data, result.status, result.headers);
    } else {
      ERROR_LOG("Unexpected error making API request: %o", result);
      throw new ErrorResponse({
        status: result.status,
        headers: result.headers,
        data
      });
    }
  }
  return {
    status: result.status,
    headers: result.headers,
    data
  };
}

/**
 * Set the access token that we are going to use for future requests
 * @param accessToken The access token, or `undefined` to clear it
 */
export function setAccessToken(accessToken: string | undefined) {
  _accessToken = accessToken;
}
