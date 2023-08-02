// See how the options work here: https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API

import { fs, invoke, path } from "@tauri-apps/api";
import { BaseDirectory } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";



let options = {
    rootMargin: "500px 0%",
}

const observer = new IntersectionObserver(entries => {
    for (let entry of entries) {
        if (entry.isIntersecting) {

            
            let container = (entry.target as HTMLElement);
            let id = container.dataset.id;
            let filename = container.dataset.filename;

            observer.unobserve(container);

            invoke("generate_thumbnail", {id, filename}).then((path: string) => {
                
                container.style.backgroundImage = `url('${convertFileSrc(path)}')`;

            });
    
        }
    }

}, options)

export const loadThumbnail = (container: HTMLElement,[id, filename]) => {

    container.dataset.id = id;
    container.dataset.filename = filename;


    observer.observe(container)  // intersection observer

    return {
        destroy() {
            observer.unobserve(container)
        }
    }
}
