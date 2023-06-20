(Number.prototype as any).mod = function (n) {
    "use strict";
    return ((this % n) + n) % n;
};
