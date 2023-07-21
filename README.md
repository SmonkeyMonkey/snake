#### standard snake game built on rust+wasm


Snake game requires [Wasm-pack](https://rustwasm.github.io/wasm-pack/) to build project.
Install wasm-pack:
```sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
you can also use npm or yarn to download a precompiled binary:
```sh
npm install -g wasm-pack
```
or 
```sh
yarn global add wasm-pack
```
Installation:
```sh
git clone https://github.com/smonkeymonkey/snake
```
Build the project 
```sh
cd snake
wasm-pack build --target web
```
Run server with python:
```sh
python3 -m http.server
```
Open in browser:
```sh
127.0.0.1:8000
```
