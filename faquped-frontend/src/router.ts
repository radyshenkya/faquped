import HttpStatusCode from "./http_status_codes";

type RouteCallbackProps = {
    path: string,
    path_params: Record<string, string>,
};

type RouteError = {
    httpStatusCode: HttpStatusCode,
    message: string,
};

type RouteErrorHandler = (err: RouteError) => void;
type RouteHandler = (props: RouteCallbackProps) => void;
type Routes = Record<string, RouteHandler>;

export class Router {
    routes: Routes = {};
    error_route: RouteErrorHandler = (_err: RouteError) => console.error("Error handler is not specified");

    constructor(routes: Routes, error_route: RouteErrorHandler) {
        this.routes = routes;
        this.error_route = error_route;
    }
}

export function parsePathByTemplate(template: string, path: string): RouteCallbackProps | null {
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

    let routeProps: RouteCallbackProps = { path: path, path_params: {} };

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

