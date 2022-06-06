require.config({
  paths: { vs: "./js/vs" },
  // "js/vs/css": { disabled: true },
});

const container = document.getElementById("editor");
const shadowRoot = container.attachShadow({
  mode: "closed",
});

const innerContainer = document.createElement("div");
shadowRoot.appendChild(innerContainer);
innerContainer.style.width = "100%";
innerContainer.style.height = "600px";

const innerStyle = document.createElement("style");
innerStyle.innerText = '@import "./js/vs/editor/editor.main.css";';
shadowRoot.appendChild(innerStyle);

require(["vs/editor/editor.main"], function () {
  const editor = monaco.editor.create(innerContainer, {
    value: `
# Title

Enter content here!
    `.trim(),
    language: "markdown",
  });
  const dropSuffix = (v) => v.replace(/\..+/, "");
  const themesEl = document.getElementById("theme-selector");
  [
    "",
    `GitHub.json`,
    `Monokai.json`,
    `Nord.json`,
    `Solarized-dark.json`,
    `Solarized-light.json`,
  ].forEach((basename) => {
    const opt = document.createElement("option");
    opt.text = dropSuffix(basename);
    opt.value = basename;
    themesEl.appendChild(opt);
  });
  themesEl.onchange = async (evt) => {
    const basename = evt.target.value;
    if (!basename) {
      return;
    }
    const lastThemesElDisabled = themesEl.disabled;
    themesEl.disabled = true;
    try {
      await fetch(`/js/monaco-themes/themes/${basename}`)
        .then((r) => r.json())
        .then((data) => {
          const themeName = dropSuffix(basename);
          monaco.editor.defineTheme(themeName, data);
          monaco.editor.setTheme(themeName);
        });
    } finally {
      themesEl.disabled = lastThemesElDisabled;
    }
  };
});
