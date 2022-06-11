require.config({
  paths: { vs: "/js/vs" },
  // "js/vs/css": { disabled: true },
});

const container = document.getElementById("editor");
const initialContentEl = document.getElementById("editor-initial");
const initialContent = initialContentEl?.textContent || "";
initialContentEl?.remove();
const shadowRoot = container.attachShadow({
  mode: "closed",
});

const innerContainer = document.createElement("div");
shadowRoot.appendChild(innerContainer);
innerContainer.style.width = "100%";
innerContainer.style.height = "600px";

const innerStyle = document.createElement("style");
innerStyle.innerText = '@import "/js/vs/editor/editor.main.css";';
shadowRoot.appendChild(innerStyle);

require(["vs/editor/editor.main"], function () {
  /**
   * API: https://microsoft.github.io/monaco-editor/api/interfaces/monaco.editor.IStandaloneCodeEditor.html
   */
  const editor = monaco.editor.create(innerContainer, {
    value:
      initialContent ||
      `
# Title

Enter content here!
    `.trim(),
    language: "markdown",
  });
  const dropSuffix = (v) => v.replace(/\..+/, "");
  const themesEl = document.getElementById("theme-selector");
  const themeJsons = [
    "",
    `GitHub.json`,
    `Monokai.json`,
    `Nord.json`,
    `Solarized-dark.json`,
    `Solarized-light.json`,
  ];
  themeJsons.forEach((basename) => {
    const opt = document.createElement("option");
    opt.text = dropSuffix(basename);
    opt.value = basename;
    themesEl.appendChild(opt);
  });
  const setTheme = async (basename) => {
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
  themesEl.onchange = async (evt) => {
    const themeBasename = evt.target.value;
    if (themeBasename) {
      await setTheme(themeBasename);
    }
  };
  // set default
  const defaultThemeBasename = themeJsons.find((f) => f.match(/nord/i));
  themesEl.value = defaultThemeBasename;
  setTheme(defaultThemeBasename);

  window.addEventListener("resize", () => {
    const { width } = window.document.body.getBoundingClientRect();
    editor.layout({
      width: width * 0.8,
      height: editor.height,
    });
    editor.layout();
  });
});
