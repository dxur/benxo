vcl 4.1;

import std;

backend default {
    .host = "backend";
    .port = "5080";
}

sub vcl_recv {
    if (req.method != "GET" && req.method != "HEAD") {
        return (pass);
    }

    if (req.http.Authorization) {
        return (pass);
    }

    if (req.url ~ "\.(png|webp|jpg|jpeg|gif|ico|css|js|woff2?|ttf|svg)$") {
        unset req.http.Cookie;
    }

    return (hash);
}

sub vcl_backend_response {
    if (beresp.ttl <= 0s || beresp.http.Set-Cookie) {
        set beresp.uncacheable = true;
        set beresp.ttl = 120s;
        return (deliver);
    }

    if (beresp.ttl <= 0s) {
        set beresp.ttl = 5m;
    }

    return (deliver);
}

sub vcl_deliver {
    if (obj.hits > 0) {
        set resp.http.X-Cache = "HIT";
    } else {
        set resp.http.X-Cache = "MISS";
    }
}
