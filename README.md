<div align="center">

  <h1><code>afrim-js</code></h1>

  <strong>A binding of the <a href="https://github.com/pythonbrad/afrim">afrim ime engine</a> for the web.</strong>

  <p>
    <a href="https://github.com/pythonbrad/afrim-js/actions/workflows/ci.yml"><img alt="Build Status" src="https://github.com/pythonbrad/afrim-js/actions/workflows/ci.yml/badge.svg?branch=main"/></a>
    <a href="https://www.npmjs.org/package/afrim-js"><img alt="NPM version" src="https://img.shields.io/npm/v/afrim-js.svg?style=flat-square"/></a>
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

### Installation

```
npm install afrim-js
```

### Usage

```javascript
import { Preprocessor, Translator } from "afrim-js";
import { convertTomlToJson } from "afrim-js";

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
  var translator = new Translator(dictionary, false);
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
