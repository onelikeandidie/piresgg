import hljs from "highlight.js/lib/core";
import rust from "highlight.js/lib/languages/rust";
import markdown from "highlight.js/lib/languages/markdown";
import twig from "highlight.js/lib/languages/twig";
import ini from "highlight.js/lib/languages/ini";

hljs.registerLanguage("rust", rust);
hljs.registerLanguage("markdown", markdown);
hljs.registerLanguage("twig", twig);
hljs.registerLanguage("ini", ini);

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

function setupLazyImages() {
    const images = document.querySelectorAll("img");
    let observer = new IntersectionObserver((entries, observer) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                let img = entry.target as HTMLImageElement;
                // Remove the lazy:// from the src
                if (img.src.startsWith("lazy://")) {
                    img.src = img.src.replace("lazy://", "");
                    img.onerror = () => {
                        img.src = "/public/assets/images/error.png";
                    }
                }
                // Stop observing the image
                observer.unobserve(img);
            }
        });
    });
    images.forEach((img) => {
        observer.observe(img);
    });
}

document.addEventListener("DOMContentLoaded", () => {
    setupCodeBlocks();
    setupLazyImages();
});