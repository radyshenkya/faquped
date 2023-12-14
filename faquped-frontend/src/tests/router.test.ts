import { parsePathByTemplate } from "../router";
import { expect, test } from "vitest";

test("Parses simple path", () => {
    let path = "/simple_path";

    let res = parsePathByTemplate(
        path,
        path,
    );
    expect(res).toEqual({ path: path, path_params: {}})
});

test("Parses empty path", () => {
    let path = "/";

    let res = parsePathByTemplate(
        path,
        path,
    );
    expect(res).toEqual({ path: path, path_params: {}})
})

test("Parses path with multiple segments", () => {
    let path = "/segment1/segment2/segment3";

    let res = parsePathByTemplate(
        path,
        path,
    );
    expect(res).toEqual({ path: path, path_params: {}})
})

test("Parses path with one parameter", () => {
    let path = "/path/:param1";

    let res = parsePathByTemplate(
        path,
        "/path/first parameter result",
    );

    expect(res).toEqual({ path: "/path/first parameter result", path_params: {"param1": "first parameter result"}})
})

test("Parses path with multiple parameters", () => {
    let path = "/segment1/:param1/segment2/:param2";

    let res = parsePathByTemplate(
        path,
        "/segment1/123/segment2/456",
    );

    expect(res).toEqual({ path: "/segment1/123/segment2/456", path_params: {"param1": "123", "param2": "456"}})
})

test("Parses path with wildcard", () => {
    let path = "/path/with/wildcard/*";

    let res = parsePathByTemplate(
        path,
        "/path/with/wildcard/this/segments/is/in/wildcard.html",
    );

    expect(res).toEqual({ path: "/path/with/wildcard/this/segments/is/in/wildcard.html", path_params: {"*": "this/segments/is/in/wildcard.html"}})
})

test("Failed to parse path with equal segments count", () => {
    let path = "/segment1/segment2/segment3";

    let res = parsePathByTemplate(
        path,
        "/not/matching/url",
    );
    expect(res).toEqual(null)
})

test("Failed to parse path with more segments", () => {
    let path = "/segment1/segment2/segment3";

    let res = parsePathByTemplate(
        path,
        path + "/segment4",
    );
    expect(res).toEqual(null)
})

test("Failed to parse path with less segments count", () => {
    let path = "/segment1/segment2/segment3";

    let res = parsePathByTemplate(
        path,
        "/segment1/segment2",
    );
    expect(res).toEqual(null)
})
