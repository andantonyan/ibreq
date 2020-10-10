class Clients {

    constructor(opts) {
        this._count = 0;
        this._clients = {};
        this._inactiveTimeout = opts.inactiveTimeout || 10000;
    }

    get count() {
        return Object.keys(this._clients).length;
    }

    get list() {
        return this._clients;
    }

    add(token) {

        this._clients[token] = setTimeout(() => {

            clearTimeout(this._clients[token]);
            this.del.call(this, token);

        }, this._inactiveTimeout);

    }

    del(token) {
        delete this._clients[token];
    }

    contains(token) {
        return (token in this.list);
    }

}

module.exports = Clients;