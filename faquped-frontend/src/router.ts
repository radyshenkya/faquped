import HttpStatusCode from "./http_status_codes";

export type RouteHandlerProps = {
    path: string,
    path_params: Record<string, string>,
};

export class RouteError {
    httpStatusCode: HttpStatusCode;
    message: string;
    additionalData: any;

    constructor(httpStatusCode: HttpStatusCode, message: string, additionalData: any) {
        this.httpStatusCode = httpStatusCode;
        this.message = message;
        this.additionalData = additionalData;
    }
};

export type RouteErrorHandler = (err: RouteError) => void;
export type RouteHandler = (props: RouteHandlerProps) => void;
export type Routes = { [path: string]: RouteHandler };

const ROUTE_NOT_FOUND_ERROR = new RouteError(
    HttpStatusCode.NOT_FOUND,
    "Path did not match any route in router",
    null,
);

export class Router {
    routes: Routes = {};
    error_route: RouteErrorHandler = (err: any) => console.error("Error handler is not specified", err);

    constructor(routes: Routes, error_route: RouteErrorHandler) {
        this.routes = routes;
        this.error_route = error_route;
    }

    route(path: string) {
        for (const [template, callback] of Object.entries(this.routes)) {
            let parsedProps = parsePathByTemplate(template, path);

            if (parsedProps == null) {
                continue;
            }

            try {
                callback(parsedProps);
            } catch (e: any) {
                this.error_route(e);
            } finally {
                return;
            }
        }

        this.error_route(ROUTE_NOT_FOUND_ERROR);
    }

    updateClickEvents() {
        const elems = document.querySelectorAll('a[data-router]');

        for (const el of elems) {
            if (el.getAttribute('router-listener') === 'true') {
                continue;
            }

            el.setAttribute('router-listener', 'true');
            el.addEventListener('click', (ev: any) => {
                ev.preventDefault();
                history.pushState({}, "", ev.target.href);
                this.route(ev.target.pathname);
            });
        }
    }
}

export function parsePathByTemplate(template: string, path: string): RouteHandlerProps | null {
    let pathSegments = path.split("/");
    let templateSegments = template.split("/");

    if (pathSegments.length < templateSegments.length) {
        return null;
    }

    let zippedSegments = templateSegments.map((segment, _i) => {
        // If wildcard detected - just consume everything
        if (segment == "*") {
            let pathSegmentsConsumed = pathSegments.join("/");
            pathSegments = [];
            return [segment, pathSegmentsConsumed];
        }

        let pathSegment = pathSegments[0] ?? null;
        pathSegments = pathSegments.slice(1);

        return [segment, pathSegment];
    });

    if (pathSegments.length != 0) {
        return null;
    }

    let routeProps: RouteHandlerProps = { path: path, path_params: {} };

    while (zippedSegments.length > 0) {
        let [templateSegment, pathSegment] = zippedSegments[0];

        if (pathSegment == null) {
            return null;
        }

        if (templateSegment.startsWith(":")) {
            routeProps.path_params[templateSegment.slice(1)] = pathSegment;
        } else if (templateSegment == "*") {
            routeProps.path_params[templateSegment] = pathSegment;
            break;
        } else if (templateSegment != pathSegment) {
            return null;
        }


        zippedSegments = zippedSegments.slice(1);

    }

    return routeProps;
}

