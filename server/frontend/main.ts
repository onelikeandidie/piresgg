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
    const images = document.querySelectorAll(".prose img");
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

function createOverlay(): HTMLElement {
    let overlay = document.createElement("div");
    overlay.id = "overlay";
    overlay.classList.add("hidden", "fixed", "top-0", "right-0", "bottom-0", "left-0", "bg-black", "bg-opacity-50", "flex", "justify-center", "items-center");
    document.body.appendChild(overlay);

    // Add some helper functions for the overlay
    overlay.addEventListener("click", (e) => {
        overlay.classList.add("hidden");
        overlay.innerHTML = "";
    });

    return overlay;
}

function getOverlay(): HTMLElement {
    let overlay = document.getElementById("overlay");
    if (!overlay) {
        overlay = createOverlay();
    }
    return overlay;
}

function createImagePopup(image: string): HTMLElement {
    let overlay = getOverlay();
    let image_wrapper = document.createElement("div");
    image_wrapper.classList.add("max-w-full", "max-h-full");
    let image_element = document.createElement("img");
    image_element.src = image;
    image_wrapper.append(image_element);
    return image_wrapper;
}

function setupImagePopup() {
    const images = document.querySelectorAll(".prose img") as NodeListOf<HTMLImageElement>;
    images.forEach((img) => {
        img.addEventListener("click", (e) => {
            let image = img.src;
            let image_element = createImagePopup(image);
            let overlay = getOverlay();
            overlay.append(image_element);
            overlay.classList.remove("hidden");
        });
    });

}

document.addEventListener("DOMContentLoaded", () => {
    setupCodeBlocks();
    setupLazyImages();
    setupImagePopup();
});