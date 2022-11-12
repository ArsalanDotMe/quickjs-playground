const module = {
  exports: {}
};

class Response {
  constructor(responseText, statusCode, headers) {
    this.responseText = responseText || "";
    this.headers = headers || {};
    this.statusCode = parseInt(statusCode, 10) || 200;
  }
};

async function callUserCode({ params }) {
  if (module.exports.onRequestGet) {
    const result = await module.exports.onRequestGet({ params: { id: 173 } });
    return JSON.stringify(result);
  }
}

async function fetch(url) {
  return new Promise((resolve, reject) => {
    const response = native_fetch(1, url, {});
    resolve({
      json: () => {
        return new Promise((resolve, reject) => {
          try {
            resolve(JSON.parse(response.raw_text));
          } catch (e) {
            reject(e);
          }
          resolve(JSON.parse(response.raw_text));
        });
      },
      text: () => Promise.resolve(response.raw_text)
    });
  });
};

