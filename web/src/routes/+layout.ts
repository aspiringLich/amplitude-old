(Number.prototype as any).mod = function (n) {
    "use strict";
    return ((this % n) + n) % n;
};

export const load = ({ url }) => {
    const { pathname } = url;

    return {
        pathname,
    };
};