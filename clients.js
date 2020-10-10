class Clients {
  constructor(opts) {
    this._count = 0;
    this._clients = {};
    this._inactiveTimeout = opts.inactiveTimeout || 10000;
  }

  get tokens() {
    return Object.keys(this._clients);
  }

  get count() {
    return this.tokens.length;
  }

  add(token) {
    if (token) {
      this._clients[token] = setTimeout(() => {
        clearTimeout(this._clients[token]);
        this.del.call(this, token);
      }, this._inactiveTimeout);
    }
  }

  del(token) {
    delete this._clients[token];
  }

  contains(token) {
    return token in this._clients;
  }
}

module.exports = Clients;
