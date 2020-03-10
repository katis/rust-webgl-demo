export function raw_load_image(source) {
    return new Promise((resolve, reject) => {
        const image = new Image();

        image.onload = () => {
            resolve(image);
        };

        image.onerror = err => {
            reject(err);
        };

        image.src = source;
    });
}