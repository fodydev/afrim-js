<div align="center">

  <h1><code>afrim</code></h1>

  <strong>A binding of the <a href="https://github.com/pythonbrad/afrim">afrim ime engine</a> for the web.</strong>

  <p>
    <a href="https://github.com/pythonbrad/afrim-js/actions/workflows/ci.yml"><img alt="Build Status" src="https://github.com/pythonbrad/afrim-js/actions/workflows/ci.yml/badge.svg?branch=main"/></a>
    <a href="https://www.npmjs.org/package/afrim"><img alt="NPM version" src="https://img.shields.io/npm/v/afrim.svg?style=flat-square"/></a>
  </p>

  <h3>
    <a href="https://github.com/pythonbrad/afrim-web">Demo</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://github.com/pythonbrad">@pythonbrad</a></sub>
</div>

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

**Note**: Always use `wasm-pack build --debug` in debug mode.

### 🔋 Features Included

* `strsim` for text similarity.
* `rhai` for scripting.

### Lite Version
We also offer a lite version of this library for those who need a more lightweight solution.

You can find it here [npm/afrim-lite](https://www.npmjs.com/package/afrim-lite).

**Note:** This version implies the dropped rhai feature.

### Installation

```
npm install afrim
```

### Serving the wasm file

The `afrim_bg.wasm` file is large (~900 KB), but it compresses very well: gzip brings it down
to ~340 KB, about 62% smaller. Serve it compressed to cut download time and speed up page
loads, especially on slow mobile connections.

Most CDNs (jsDelivr, unpkg, Cloudflare) and static hosts (GitHub Pages, Netlify, Vercel)
already do this for you. If you host the file yourself, make sure compression is enabled for
the `application/wasm` MIME type. For example:

* **nginx**: `gzip on;` and `gzip_types application/wasm;`
* **Apache**: `AddOutputFilterByType DEFLATE application/wasm`

Brotli usually compresses even better than gzip, if your server supports it.

To check, open your browser's devtools, select the `.wasm` request in the Network tab, and
look for a `Content-Encoding: gzip` (or `br`) response header.

### Usage

```javascript
import { Preprocessor, Translator } from "afrim";
import { convertTomlToJson } from "afrim";

(async function () {
  // We execute preprocessor commands in idle.
  var processCommand = () => {
    var cmd = JSON.parse(preprocessor.popQueue());
    // ...
    requestAnimationFrame(processCommand);
  };
  // ...

  // We config the afrim ime.
  var preprocessor = new Preprocessor(data, 64);
  var translator = new Translator(dictionary, false, 0.7);
  Object.entries(scripts).forEach((e) =>
    translator.register(e[0], e[1]),
  );
  // ...

  // We listen keyboard events.
  textFieldElement.addEventListener(
    "keyup",
    (event) => {
      // ...
      
      // Commit the predicate.
      if (event.code == "Space") {
        var predicate = global.memory.predicates[global.memory.predicateId];

        if (predicate) preprocessor.commit(predicate[3]);
        clearPredicate();
      }

      var changed = preprocessor.process(event.key, "keydown");
      var input = preprocessor.getInput();

      // We update the predicates
      if (!changed) return;

      tooltipInputElement.innerText = "📝 " + input;

      var predicates = translator.translate(input);
      loadPredicates(predicates);
      updatePredicate();
      // ...
    },
    false,
  );
  // ...

  // We start the processor.
  requestAnimationFrame(processCommand);
})();
```

### License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

### Contribution

We are open for contributions.
