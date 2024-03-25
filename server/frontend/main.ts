import hljs from "highlight.js/lib/core";
import rust from "highlight.js/lib/languages/rust";

hljs.registerLanguage("rust", rust);

function setupCodeBlocks() {
    const codeBlocks = document.querySelectorAll(".prose pre code");
    codeBlocks.forEach((block: HTMLElement) => {
        // Get the class that starts with "language-"
        let classes = block.className.split(" ");
        console.log(classes)
        let languageClass = classes.find((c) => c.startsWith("language-"));
        if (!languageClass) {
            return;
        }
        // Get the language name
        let language = languageClass.replace("language-", "");
        console.log(language)
        let html = hljs.highlight(
            block.innerText,
            { language: language }
        );
        console.log(html)
        block.innerHTML = html.value;
    });
}

document.addEventListener("DOMContentLoaded", () => {
    setupCodeBlocks();
});