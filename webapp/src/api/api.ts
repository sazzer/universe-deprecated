import * as rfc6570 from "rfc6570-expand";

import { Problem, ProblemResponse } from "./problem";
import axios, { AxiosResponse, Method } from "axios";

import debug from "debug";

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
export interface Request {
  url: string;
  urlParams?: any;
  method?: Method;
  headers?: any;
  data?: any;
}

/**
 * The shape of a Response from a request
 */
export type Response<T> = AxiosResponse<T>;

/**
 * Build an API requester to make API calls with
 */
export async function request<T>(request: Request): Promise<Response<T>> {
  let serviceUrl = process.env.REACT_APP_SERVICE_URL;
  if (window && window._UNIVERSE_CONFIG && window._UNIVERSE_CONFIG.serviceUrl) {
    serviceUrl = window._UNIVERSE_CONFIG.serviceUrl;
  }

  const template = rfc6570.init(request.url);
  const expandedUri = template.expand(request.urlParams || {});
  LOG("Making request to: %s", expandedUri);

  const headers = {
    ...request.headers
  };

  if (_accessToken) {
    headers.Authorization = `Bearer ${_accessToken}`;
  }

  try {
    return await axios.request({
      baseURL: serviceUrl,
      timeout: 20000,
      url: expandedUri,
      method: request.method,
      headers: headers,
      data: request.data
    });
  } catch (e) {
    if (e.response) {
      const response = e.response as AxiosResponse<Problem>;
      if (response.headers["content-type"] === "application/problem+json") {
        LOG("Received an RFC-7807 Problem response: %o", response.data);
        throw new ProblemResponse(
          response.data,
          response.status,
          response.headers
        );
      }
    }

    ERROR_LOG("Unexpected error making API request", e);
    throw e;
  }
}

/**
 * Set the access token that we are going to use for future requests
 * @param accessToken The access token, or `undefined` to clear it
 */
export function setAccessToken(accessToken: string | undefined) {
  _accessToken = accessToken;
}
