from flask import Flask, request, redirect, Response
import requests
import ssl


app = Flask(__name__)
SITE_NAME = 'http://0.0.0.0/'
bind_ip = '0.0.0.0'
https_port = 443




@app.route('/<path:path>', methods=['GET', 'POST', 'DELETE'])
def proxy(path):
    global SITE_NAME
    if request.method =='GET':
        resp = requests.get(f'{SITE_NAME}{path}')
        excluded_headers = ['content-encoding', 'content-length', 'transfer-encoding', 'connection']
        headers = [(name, value) for (name, value) in resp.raw.headers.items(
        ) if name.lower() not in excluded_headers]
        response = Response(resp.content, resp.status_code, headers)
        return response
    elif request.method =='POST':
        resp = requests.post(f'{SITE_NAME}{path}', json=request.get_json())
        excluded_headers = ['content-encoding', 'content-length', 'transfer-encoding', 'connection']
        headers = [(name, value) for (name, value) in resp.raw.headers.items(
        ) if name.lower() not in excluded_headers]
        response = Response(resp.content, resp.status_code, headers)
        return response
    elif request.method =='DELETE':
        resp = requests.delete(f'{SITE_NAME}{path}').content
        response = Response(resp.content, resp.status_code, headers)
        return response


if __name__ == '__main__':
    ctx = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
    ctx.load_cert_chain(certfile="ssl/wat.pem")
    ctx.options |= ssl.OP_CIPHER_SERVER_PREFERENCE
    ctx.set_ciphers('ECDHE-ECDSA-AES128-GCM-SHA256')
    ctx.set_ecdh_curve('prime256v1')
    app.run(host=bind_ip, port=https_port, ssl_context=ctx)
