vcl 4.1;

import std;

backend default {
    .host = "backend";
    .port = "5080";
}

// TODO: setup proper caching
sub vcl_recv {
    return(pass);
}
